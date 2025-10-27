use anyhow::Result;

use crate::validate_invariant;

/// Event emitted when a participant successfully registers.
#[derive(Debug, Clone)]
pub struct ParticipantRegistered {
    pub participant_id: String,
    pub email: String,
}

impl ParticipantRegistered {
    pub fn validate(&self) -> Result<()> {
        validate_invariant(!self.participant_id.trim().is_empty())?;
        validate_invariant(self.email.contains('@'))?;
        Ok(())
    }
}

/// Event emitted when a team is created.
#[derive(Debug, Clone)]
pub struct TeamCreated {
    pub team_id: String,
    pub hackathon_id: String,
}

impl TeamCreated {
    pub fn validate(&self) -> Result<()> {
        validate_invariant(!self.team_id.trim().is_empty())?;
        validate_invariant(!self.hackathon_id.trim().is_empty())?;
        Ok(())
    }
}

/// Event emitted when a new member joins a team.
#[derive(Debug, Clone)]
pub struct TeamMemberJoined {
    pub team_id: String,
    pub participant_id: String,
}

impl TeamMemberJoined {
    pub fn validate(&self) -> Result<()> {
        validate_invariant(!self.team_id.trim().is_empty())?;
        validate_invariant(!self.participant_id.trim().is_empty())?;
        Ok(())
    }
}

/// Event emitted when a solution is submitted.
#[derive(Debug, Clone)]
pub struct SolutionSubmitted {
    pub submission_id: String,
    pub team_id: String,
}

impl SolutionSubmitted {
    pub fn validate(&self) -> Result<()> {
        validate_invariant(!self.submission_id.trim().is_empty())?;
        validate_invariant(!self.team_id.trim().is_empty())?;
        Ok(())
    }
}

/// Event emitted when a submission is rated.
#[derive(Debug, Clone)]
pub struct SubmissionRated {
    pub rating_id: String,
    pub submission_id: String,
    pub score: u8,
}

impl SubmissionRated {
    pub fn validate(&self) -> Result<()> {
        validate_invariant(!self.rating_id.trim().is_empty())?;
        validate_invariant((1..=10).contains(&self.score))?;
        Ok(())
    }
}
