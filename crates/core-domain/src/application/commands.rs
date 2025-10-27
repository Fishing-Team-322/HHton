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
    pub repository_binding: SubmitSolutionRepositoryBinding,
}

impl SubmitSolution {
    pub fn validate(&self) -> Result<()> {
        validate_invariant(!self.submission_id.trim().is_empty())?;
        validate_invariant(!self.team_id.trim().is_empty())?;
        validate_invariant(!self.hackathon_id.trim().is_empty())?;
        validate_invariant(!self.summary.trim().is_empty())?;
        self.repository_binding.validate()?;
        Ok(())
    }
}

/// Repository binding details for a submission command.
#[derive(Debug, Clone)]
pub struct SubmitSolutionRepositoryBinding {
    pub provider: String,
    pub repository: String,
    pub commit: String,
    pub is_private: bool,
}

impl SubmitSolutionRepositoryBinding {
    pub fn validate(&self) -> Result<()> {
        validate_invariant(!self.provider.trim().is_empty())?;
        validate_invariant(!self.repository.trim().is_empty())?;
        validate_invariant(!self.commit.trim().is_empty())?;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_repository_binding() -> SubmitSolutionRepositoryBinding {
        SubmitSolutionRepositoryBinding {
            provider: "github".into(),
            repository: "owner/repo".into(),
            commit: "abc123".into(),
            is_private: false,
        }
    }

    #[test]
    fn submit_solution_requires_team_id() {
        let command = SubmitSolution {
            submission_id: "sub-1".into(),
            team_id: " ".into(),
            hackathon_id: "hack-1".into(),
            summary: "A great project".into(),
            repository_binding: valid_repository_binding(),
        };

        assert!(command.validate().is_err());
    }

    #[test]
    fn submit_solution_requires_hackathon_id() {
        let command = SubmitSolution {
            submission_id: "sub-1".into(),
            team_id: "team-1".into(),
            hackathon_id: "".into(),
            summary: "A great project".into(),
            repository_binding: valid_repository_binding(),
        };

        assert!(command.validate().is_err());
    }

    #[test]
    fn submit_solution_requires_repository_binding_fields() {
        let mut binding = valid_repository_binding();
        binding.provider.clear();

        let command = SubmitSolution {
            submission_id: "sub-1".into(),
            team_id: "team-1".into(),
            hackathon_id: "hack-1".into(),
            summary: "A great project".into(),
            repository_binding: binding,
        };

        assert!(command.validate().is_err());
    }

    #[test]
    fn submit_solution_requires_commit_reference() {
        let mut binding = valid_repository_binding();
        binding.commit = "  ".into();

        let command = SubmitSolution {
            submission_id: "sub-1".into(),
            team_id: "team-1".into(),
            hackathon_id: "hack-1".into(),
            summary: "A great project".into(),
            repository_binding: binding,
        };

        assert!(command.validate().is_err());
    }
}
