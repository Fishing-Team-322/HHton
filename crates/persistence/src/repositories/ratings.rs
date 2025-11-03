use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use chrono::NaiveDate;
use ratings::{EventOutcome, EventScale, ParticipantProfile};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
struct ParticipantProfileRecord {
    participant_id: String,
    team_id: Option<String>,
    rating: f64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct EventOutcomeRecord {
    participant_id: String,
    #[allow(dead_code)]
    team_id: Option<String>,
    position: i32,
    total_participants: i32,
    scale: i16,
    finished_at: NaiveDate,
    team_size: i32,
}

fn scale_to_i16(scale: EventScale) -> i16 {
    match scale {
        EventScale::Local => 1,
        EventScale::Regional => 2,
        EventScale::National => 3,
        EventScale::Global => 4,
    }
}

fn scale_from_i16(value: i16) -> Result<EventScale> {
    match value {
        1 => Ok(EventScale::Local),
        2 => Ok(EventScale::Regional),
        3 => Ok(EventScale::National),
        4 => Ok(EventScale::Global),
        other => Err(anyhow!("unknown rating scale: {other}")),
    }
}

impl TryFrom<EventOutcomeRecord> for EventOutcome {
    type Error = anyhow::Error;

    fn try_from(value: EventOutcomeRecord) -> Result<Self> {
        let scale = scale_from_i16(value.scale)?;
        let position = u32::try_from(value.position)
            .context("event outcome position must be positive and within range")?;
        let total_participants = u32::try_from(value.total_participants)
            .context("event outcome total participants must be positive and within range")?;
        let team_size = u32::try_from(value.team_size)
            .context("event outcome team size must be positive and within range")?;

        EventOutcome::new(
            position,
            total_participants,
            scale,
            value.finished_at,
            team_size,
        )
    }
}

/// Data access layer for rating profiles and their outcomes.
#[derive(Debug, Clone)]
pub struct RatingRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> RatingRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn fetch_participant_profile(
        &self,
        participant_id: &str,
    ) -> Result<Option<ParticipantProfile>> {
        let record = sqlx::query_as::<_, ParticipantProfileRecord>(
            r#"SELECT participant_id, team_id, rating
            FROM rating_participant_profiles
            WHERE participant_id = $1"#,
        )
        .bind(participant_id)
        .fetch_optional(self.pool)
        .await?;

        let Some(record) = record else {
            return Ok(None);
        };

        let event_rows = sqlx::query_as::<_, EventOutcomeRecord>(
            r#"SELECT participant_id, team_id, position, total_participants, scale, finished_at, team_size
            FROM rating_event_outcomes
            WHERE participant_id = $1
            ORDER BY finished_at ASC, recorded_at ASC"#,
        )
        .bind(participant_id)
        .fetch_all(self.pool)
        .await?;

        let mut events = Vec::with_capacity(event_rows.len());
        for row in event_rows {
            events.push(EventOutcome::try_from(row)?);
        }

        Ok(Some(ParticipantProfile {
            participant_id: record.participant_id,
            team_id: record.team_id,
            events,
            rating: record.rating,
        }))
    }

    pub async fn fetch_all_participant_profiles(&self) -> Result<Vec<ParticipantProfile>> {
        let records = sqlx::query_as::<_, ParticipantProfileRecord>(
            r#"SELECT participant_id, team_id, rating FROM rating_participant_profiles"#,
        )
        .fetch_all(self.pool)
        .await?;

        let event_rows = sqlx::query_as::<_, EventOutcomeRecord>(
            r#"SELECT participant_id, team_id, position, total_participants, scale, finished_at, team_size
            FROM rating_event_outcomes
            WHERE participant_id IS NOT NULL
            ORDER BY finished_at ASC, recorded_at ASC"#,
        )
        .fetch_all(self.pool)
        .await?;

        let mut grouped_events: HashMap<String, Vec<EventOutcome>> = HashMap::new();
        for row in event_rows {
            let event = EventOutcome::try_from(row.clone())?;
            grouped_events
                .entry(row.participant_id)
                .or_default()
                .push(event);
        }

        let mut profiles = Vec::with_capacity(records.len());
        for record in records {
            let events = grouped_events
                .remove(&record.participant_id)
                .unwrap_or_default();
            profiles.push(ParticipantProfile {
                participant_id: record.participant_id,
                team_id: record.team_id,
                events,
                rating: record.rating,
            });
        }

        Ok(profiles)
    }

    pub async fn persist_participant_profile(
        &self,
        profile: &ParticipantProfile,
        new_event: &EventOutcome,
    ) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            r#"INSERT INTO rating_participant_profiles (participant_id, team_id, rating, updated_at)
            VALUES ($1, $2, $3, NOW())
            ON CONFLICT (participant_id) DO UPDATE SET
                team_id = EXCLUDED.team_id,
                rating = EXCLUDED.rating,
                updated_at = NOW()"#,
        )
        .bind(&profile.participant_id)
        .bind(&profile.team_id)
        .bind(profile.rating)
        .execute(&mut *tx)
        .await?;

        let position =
            i32::try_from(new_event.position).context("position exceeds supported range")?;
        let total_participants = i32::try_from(new_event.total_participants)
            .context("total participants exceeds supported range")?;
        let team_size =
            i32::try_from(new_event.team_size).context("team size exceeds supported range")?;
        let scale = scale_to_i16(new_event.scale);

        sqlx::query(
            r#"INSERT INTO rating_event_outcomes
                (id, participant_id, team_id, position, total_participants, scale, finished_at, team_size, recorded_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())"#,
        )
        .bind(Uuid::new_v4())
        .bind(&profile.participant_id)
        .bind(&profile.team_id)
        .bind(position)
        .bind(total_participants)
        .bind(scale)
        .bind(new_event.finished_at)
        .bind(team_size)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::RatingRepository;
    use chrono::NaiveDate;
    use ratings::{
        update_participant_profile, EventOutcome, EventScale, ParticipantProfile, RatingFactors,
    };
    use sqlx::PgPool;

    #[sqlx::test(migrations = "./migrations")]
    async fn persist_and_reload_participant_profile(pool: PgPool) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO users (id, display_name, email) VALUES ($1, $2, $3)")
            .bind("user-1")
            .bind("Alice")
            .bind("alice@example.com")
            .execute(&pool)
            .await?;

        let repo = RatingRepository::new(&pool);
        let mut profile = ParticipantProfile::new("user-1", None);
        let outcome = EventOutcome::new(
            1,
            20,
            EventScale::Global,
            NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(),
            1,
        )
        .unwrap();
        let factors = RatingFactors::default();
        let today = NaiveDate::from_ymd_opt(2024, 7, 2).unwrap();
        update_participant_profile(&mut profile, outcome.clone(), &factors, today).unwrap();

        repo.persist_participant_profile(&profile, &outcome)
            .await
            .unwrap();

        let fetched = repo
            .fetch_participant_profile("user-1")
            .await
            .unwrap()
            .expect("profile stored");
        assert_eq!(fetched.participant_id, "user-1");
        assert_eq!(fetched.events.len(), 1);

        let all = repo.fetch_all_participant_profiles().await.unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].events.len(), 1);

        Ok(())
    }
}
