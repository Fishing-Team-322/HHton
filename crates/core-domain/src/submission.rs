use std::time::SystemTime;

use anyhow::Result;

use crate::{
    model::{AggregateRoot, EntityId},
    validate_invariant,
};

/// Aggregate representing a solution submitted by a team.
#[derive(Debug, Clone)]
pub struct Submission {
    id: EntityId,
    team_id: EntityId,
    hackathon_id: EntityId,
    summary: SubmissionSummary,
    created_at: SystemTime,
}

impl Submission {
    pub fn new(
        id: EntityId,
        team_id: EntityId,
        hackathon_id: EntityId,
        summary: SubmissionSummary,
        created_at: SystemTime,
    ) -> Result<Self> {
        let now = SystemTime::now();
        validate_invariant(created_at <= now)?;
        Ok(Self {
            id,
            team_id,
            hackathon_id,
            summary,
            created_at,
        })
    }

    pub fn team_id(&self) -> &EntityId {
        &self.team_id
    }

    pub fn hackathon_id(&self) -> &EntityId {
        &self.hackathon_id
    }

    pub fn summary(&self) -> &SubmissionSummary {
        &self.summary
    }

    pub fn created_at(&self) -> SystemTime {
        self.created_at
    }
}

impl AggregateRoot for Submission {
    fn id(&self) -> &str {
        &self.id.0
    }
}

/// Value object encapsulating the public-facing summary of the submission.
#[derive(Debug, Clone)]
pub struct SubmissionSummary(String);

impl SubmissionSummary {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        validate_invariant(!value.trim().is_empty())?;
        validate_invariant(value.len() <= 500)?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    #[test]
    fn summary_cannot_be_empty() {
        assert!(SubmissionSummary::new("").is_err());
        assert!(SubmissionSummary::new("  ").is_err());
    }

    #[test]
    fn summary_limited_in_length() {
        let too_long = "a".repeat(501);
        assert!(SubmissionSummary::new(too_long).is_err());
    }

    #[test]
    fn creation_time_cannot_be_in_the_future() {
        let summary = SubmissionSummary::new("Great project").unwrap();
        let future = SystemTime::now() + Duration::from_secs(60);
        let result = Submission::new(
            EntityId("submission-1".into()),
            EntityId("team-1".into()),
            EntityId("hack-1".into()),
            summary,
            future,
        );
        assert!(result.is_err());
    }
}
