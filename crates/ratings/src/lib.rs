//! Rating aggregates and supporting types.

use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::NaiveDate;

/// Represents a user generated rating for a repository artifact.
#[derive(Debug, Clone, PartialEq)]
pub struct Rating {
    score: u8,
}

impl Rating {
    /// Construct a new rating ensuring the score is within 0..=100 boundaries.
    pub fn new(score: u8) -> Result<Self> {
        anyhow::ensure!(score <= 100, "score must be between 0 and 100");
        Ok(Self { score })
    }

    /// Return the normalized floating point value for downstream services.
    pub fn normalized(&self) -> f32 {
        f32::from(self.score) / 100.0
    }

    /// Merge another rating by averaging the scores.
    pub fn merge(&mut self, other: &Rating) {
        self.score = ((self.score as u16 + other.score as u16) / 2) as u8;
    }
}

/// Helper for mapping raw inputs into ratings.
pub fn parse_score(input: &str) -> Result<Rating> {
    let value = input
        .trim()
        .parse::<u8>()
        .with_context(|| format!("invalid rating score: {input}"))?;
    Rating::new(value)
}

/// Relative scale of an event used to determine its impact on the score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventScale {
    Local,
    Regional,
    National,
    Global,
}

/// Configuration describing how individual event outcomes should be scored.
#[derive(Debug, Clone)]
pub struct RatingFactors {
    /// Base scalar that anchors all scores.
    pub base_score: f64,
    /// Exponent applied to the normalized position component.
    pub position_weight: f64,
    /// Multiplicative weights per [`EventScale`].
    pub scale_weights: HashMap<EventScale, f64>,
    /// Penalty applied per additional teammate beyond the first.
    pub team_penalty: f64,
    /// Half-life in days for exponential time decay.
    pub decay_half_life_days: f64,
}

impl RatingFactors {
    /// Retrieve the multiplier associated with a given event scale.
    pub fn scale_weight(&self, scale: EventScale) -> f64 {
        self.scale_weights.get(&scale).copied().unwrap_or(1.0)
    }
}

impl Default for RatingFactors {
    fn default() -> Self {
        let mut scale_weights = HashMap::new();
        scale_weights.insert(EventScale::Local, 0.75);
        scale_weights.insert(EventScale::Regional, 1.0);
        scale_weights.insert(EventScale::National, 1.25);
        scale_weights.insert(EventScale::Global, 1.5);

        Self {
            base_score: 100.0,
            position_weight: 1.2,
            scale_weights,
            team_penalty: 0.15,
            decay_half_life_days: 365.0,
        }
    }
}

/// Outcome information that feeds the rating algorithm.
#[derive(Debug, Clone, PartialEq)]
pub struct EventOutcome {
    /// Final placement of the participant (1 indicates a win).
    pub position: u32,
    /// Total number of teams/participants in the event.
    pub total_participants: u32,
    /// Scale describing the breadth of the event.
    pub scale: EventScale,
    /// Date when the event concluded.
    pub finished_at: NaiveDate,
    /// Number of members in the participant's team.
    pub team_size: u32,
}

impl EventOutcome {
    /// Construct a new outcome validating the essential invariants.
    pub fn new(
        position: u32,
        total_participants: u32,
        scale: EventScale,
        finished_at: NaiveDate,
        team_size: u32,
    ) -> Result<Self> {
        anyhow::ensure!(position >= 1, "position must be at least 1");
        anyhow::ensure!(
            total_participants >= position,
            "position cannot exceed total participants"
        );
        anyhow::ensure!(
            total_participants > 0,
            "total_participants must be positive"
        );
        anyhow::ensure!(team_size > 0, "team_size must be positive");

        Ok(Self {
            position,
            total_participants,
            scale,
            finished_at,
            team_size,
        })
    }
}

/// Represents a computed leaderboard entry.
#[derive(Debug, Clone, PartialEq)]
pub struct LeaderboardEntry {
    pub subject_id: String,
    pub score: f64,
}

/// Aggregated participant statistics used by external systems.
#[derive(Debug, Clone, PartialEq)]
pub struct ParticipantProfile {
    pub participant_id: String,
    pub team_id: Option<String>,
    pub events: Vec<EventOutcome>,
    pub rating: f64,
}

impl ParticipantProfile {
    pub fn new(participant_id: impl Into<String>, team_id: Option<String>) -> Self {
        Self {
            participant_id: participant_id.into(),
            team_id,
            events: Vec::new(),
            rating: 0.0,
        }
    }
}

/// Aggregated team statistics used to drive team leaderboards.
#[derive(Debug, Clone, PartialEq)]
pub struct TeamProfile {
    pub team_id: String,
    pub members: Vec<String>,
    pub events: Vec<EventOutcome>,
    pub rating: f64,
}

impl TeamProfile {
    pub fn new(team_id: impl Into<String>) -> Self {
        Self {
            team_id: team_id.into(),
            members: Vec::new(),
            events: Vec::new(),
            rating: 0.0,
        }
    }
}

/// Calculate the rating score for an event outcome.
///
/// The formula is: `base_score * (pos_norm ^ position_weight) * scale_weight * decay * team_modifier`,
/// where `pos_norm = 1 - (position - 1) / total_participants`, `decay = 0.5^(days_since / half_life)`,
/// and `team_modifier = 1 / (1 + team_penalty * max(team_size - 1, 0))`.
pub fn calculate_score(
    outcome: &EventOutcome,
    factors: &RatingFactors,
    reference_date: NaiveDate,
) -> Result<f64> {
    anyhow::ensure!(
        outcome.total_participants > 0,
        "total_participants must be positive"
    );
    anyhow::ensure!(outcome.position >= 1, "position must be at least 1");
    anyhow::ensure!(
        outcome.position <= outcome.total_participants,
        "position cannot exceed total participants",
    );
    anyhow::ensure!(outcome.team_size > 0, "team_size must be positive");

    let normalized_position = 1.0
        - (f64::from(outcome.position.saturating_sub(1)) / f64::from(outcome.total_participants));
    let position_component = normalized_position.powf(factors.position_weight);
    let scale_component = factors.scale_weight(outcome.scale);

    let days_since = reference_date
        .signed_duration_since(outcome.finished_at)
        .num_days();
    let elapsed_days = if days_since < 0 {
        0.0
    } else {
        days_since as f64
    };
    let half_life = factors.decay_half_life_days.max(f64::EPSILON);
    let decay_component = 0.5f64.powf(elapsed_days / half_life);

    let team_penalty = factors.team_penalty.max(0.0);
    let team_component =
        1.0 / (1.0 + team_penalty * f64::from(outcome.team_size.saturating_sub(1)));

    Ok(
        factors.base_score
            * position_component
            * scale_component
            * decay_component
            * team_component,
    )
}

/// Aggregate leaderboard entries for the provided subjects.
pub fn aggregate_leaderboard<'a, I>(
    records: I,
    factors: &RatingFactors,
    reference_date: NaiveDate,
) -> Result<Vec<LeaderboardEntry>>
where
    I: IntoIterator<Item = (&'a str, &'a [EventOutcome])>,
{
    let mut entries = Vec::new();

    for (subject_id, outcomes) in records.into_iter() {
        let mut score = 0.0;
        for outcome in outcomes {
            score += calculate_score(outcome, factors, reference_date)?;
        }

        entries.push(LeaderboardEntry {
            subject_id: subject_id.to_owned(),
            score,
        });
    }

    entries.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    Ok(entries)
}

/// Update the profile of a participant after a new event.
pub fn update_participant_profile(
    profile: &mut ParticipantProfile,
    new_outcome: EventOutcome,
    factors: &RatingFactors,
    reference_date: NaiveDate,
) -> Result<()> {
    profile.events.push(new_outcome);
    profile.rating = profile.events.iter().try_fold(0.0f64, |acc, event| {
        calculate_score(event, factors, reference_date).map(|score| acc + score)
    })?;
    Ok(())
}

/// Update the profile of a team and optionally manage membership.
pub fn update_team_profile(
    profile: &mut TeamProfile,
    new_outcome: EventOutcome,
    new_members: impl IntoIterator<Item = String>,
    factors: &RatingFactors,
    reference_date: NaiveDate,
) -> Result<()> {
    profile.events.push(new_outcome);
    profile.members.extend(new_members);
    profile.members.sort();
    profile.members.dedup();
    profile.rating = profile.events.iter().try_fold(0.0f64, |acc, event| {
        calculate_score(event, factors, reference_date).map(|score| acc + score)
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_high_score_for_global_win() {
        let factors = RatingFactors::default();
        let reference_date = NaiveDate::from_ymd_opt(2024, 5, 1).unwrap();
        let outcome = EventOutcome::new(
            1,
            150,
            EventScale::Global,
            NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            3,
        )
        .unwrap();

        let score = calculate_score(&outcome, &factors, reference_date).unwrap();
        assert!(
            score > 100.0,
            "expected major win to have high score, got {score}"
        );
    }

    #[test]
    fn decays_old_results() {
        let mut factors = RatingFactors::default();
        factors.decay_half_life_days = 180.0;
        let reference_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        let recent = EventOutcome::new(
            2,
            50,
            EventScale::Regional,
            NaiveDate::from_ymd_opt(2023, 12, 15).unwrap(),
            2,
        )
        .unwrap();

        let old = EventOutcome::new(
            2,
            50,
            EventScale::Regional,
            NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            2,
        )
        .unwrap();

        let recent_score = calculate_score(&recent, &factors, reference_date).unwrap();
        let old_score = calculate_score(&old, &factors, reference_date).unwrap();

        assert!(
            recent_score > old_score,
            "recent score {recent_score} should exceed old {old_score}"
        );
    }

    #[test]
    fn penalizes_large_teams() {
        let factors = RatingFactors::default();
        let reference_date = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();

        let solo = EventOutcome::new(
            3,
            40,
            EventScale::National,
            NaiveDate::from_ymd_opt(2024, 5, 1).unwrap(),
            1,
        )
        .unwrap();
        let large_team = EventOutcome::new(
            3,
            40,
            EventScale::National,
            NaiveDate::from_ymd_opt(2024, 5, 1).unwrap(),
            8,
        )
        .unwrap();

        let solo_score = calculate_score(&solo, &factors, reference_date).unwrap();
        let team_score = calculate_score(&large_team, &factors, reference_date).unwrap();

        assert!(
            solo_score > team_score,
            "solo score {solo_score} expected to exceed team score {team_score}"
        );
    }

    #[test]
    fn aggregates_leaderboard_and_updates_profiles() {
        let factors = RatingFactors::default();
        let reference_date = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();

        let mut alice_profile = ParticipantProfile::new("alice", Some("team-1".into()));
        let mut team_profile = TeamProfile::new("team-1");

        let hackathon = EventOutcome::new(
            1,
            120,
            EventScale::Global,
            NaiveDate::from_ymd_opt(2024, 6, 10).unwrap(),
            4,
        )
        .unwrap();
        update_participant_profile(
            &mut alice_profile,
            hackathon.clone(),
            &factors,
            reference_date,
        )
        .unwrap();
        update_team_profile(
            &mut team_profile,
            hackathon,
            ["alice".to_string(), "bob".to_string()],
            &factors,
            reference_date,
        )
        .unwrap();

        let smaller_meetup = EventOutcome::new(
            5,
            30,
            EventScale::Local,
            NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            2,
        )
        .unwrap();

        let leaderboard = aggregate_leaderboard(
            [
                ("alice", alice_profile.events.as_slice()),
                ("carol", &[smaller_meetup]),
            ],
            &factors,
            reference_date,
        )
        .unwrap();

        assert!(leaderboard[0].score >= alice_profile.rating);
        assert_eq!(leaderboard[0].subject_id, "alice");
        assert!(leaderboard[0].score > leaderboard[1].score);
        assert!(team_profile.members.contains(&"alice".to_string()));
        assert!(team_profile.members.contains(&"bob".to_string()));
        assert!(team_profile.rating > 0.0);
    }
}
