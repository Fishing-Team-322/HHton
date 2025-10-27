use anyhow::Result;

use crate::validate_invariant;

/// Command to register a new participant in the platform.
#[derive(Debug, Clone)]
pub struct RegisterParticipant {
    pub participant_id: String,
    pub display_name: String,
    pub email: String,
}

impl RegisterParticipant {
    pub fn validate(&self) -> Result<()> {
        validate_invariant(!self.participant_id.trim().is_empty())?;
        validate_invariant(!self.display_name.trim().is_empty())?;
        validate_invariant(self.email.contains('@'))?;
        Ok(())
    }
}

/// Command to create a new team under a hackathon.
#[derive(Debug, Clone)]
pub struct CreateTeam {
    pub team_id: String,
    pub hackathon_id: String,
    pub name: String,
    pub owner_id: String,
}

impl CreateTeam {
    pub fn validate(&self) -> Result<()> {
        validate_invariant(!self.team_id.trim().is_empty())?;
        validate_invariant(!self.hackathon_id.trim().is_empty())?;
        validate_invariant(!self.name.trim().is_empty())?;
        validate_invariant(!self.owner_id.trim().is_empty())?;
        Ok(())
    }
}

/// Command to request joining a team.
#[derive(Debug, Clone)]
pub struct JoinTeam {
    pub team_id: String,
    pub participant_id: String,
}

impl JoinTeam {
    pub fn validate(&self) -> Result<()> {
        validate_invariant(!self.team_id.trim().is_empty())?;
        validate_invariant(!self.participant_id.trim().is_empty())?;
        Ok(())
    }
}

/// Command used by teams to submit their solution.
#[derive(Debug, Clone)]
pub struct SubmitSolution {
    pub submission_id: String,
    pub team_id: String,
    pub hackathon_id: String,
    pub summary: String,
}

impl SubmitSolution {
    pub fn validate(&self) -> Result<()> {
        validate_invariant(!self.submission_id.trim().is_empty())?;
        validate_invariant(!self.summary.trim().is_empty())?;
        Ok(())
    }
}

/// Command used by judges to rate a submission.
#[derive(Debug, Clone)]
pub struct RateSubmission {
    pub rating_id: String,
    pub submission_id: String,
    pub judge_id: String,
    pub score: u8,
}

impl RateSubmission {
    pub fn validate(&self) -> Result<()> {
        validate_invariant(!self.rating_id.trim().is_empty())?;
        validate_invariant((1..=10).contains(&self.score))?;
        Ok(())
    }
}
