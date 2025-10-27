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
    repository_binding: RepositoryBinding,
    created_at: SystemTime,
}

impl Submission {
    pub fn new(
        id: EntityId,
        team_id: EntityId,
        hackathon_id: EntityId,
        summary: SubmissionSummary,
        repository_binding: RepositoryBinding,
        created_at: SystemTime,
    ) -> Result<Self> {
        let now = SystemTime::now();
        validate_invariant(created_at <= now)?;
        Ok(Self {
            id,
            team_id,
            hackathon_id,
            summary,
            repository_binding,
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

    pub fn repository_binding(&self) -> &RepositoryBinding {
        &self.repository_binding
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryBinding {
    provider: String,
    repository: String,
    reference: String,
    is_private: bool,
}

impl RepositoryBinding {
    pub fn new(
        provider: impl Into<String>,
        repository: impl Into<String>,
        reference: impl Into<String>,
        is_private: bool,
    ) -> Result<Self> {
        let provider = provider.into();
        let repository = repository.into();
        let reference = reference.into();

        validate_invariant(!provider.trim().is_empty())?;
        validate_invariant(!repository.trim().is_empty())?;
        validate_invariant(!reference.trim().is_empty())?;

        Ok(Self {
            provider,
            repository,
            reference,
            is_private,
        })
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }

    pub fn repository(&self) -> &str {
        &self.repository
    }

    pub fn reference(&self) -> &str {
        &self.reference
    }

    pub fn is_private(&self) -> bool {
        self.is_private
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
        let binding = RepositoryBinding::new("github", "owner/repo", "abc123", false).unwrap();
        let result = Submission::new(
            EntityId("submission-1".into()),
            EntityId("team-1".into()),
            EntityId("hack-1".into()),
            summary,
            binding,
            future,
        );
        assert!(result.is_err());
    }

    #[test]
    fn repository_binding_requires_all_fields() {
        assert!(RepositoryBinding::new("", "owner/repo", "main", false).is_err());
        assert!(RepositoryBinding::new("github", "", "main", false).is_err());
        assert!(RepositoryBinding::new("github", "owner/repo", "", false).is_err());
    }

    #[test]
    fn submission_can_expose_repository_binding() {
        let summary = SubmissionSummary::new("Great project").unwrap();
        let binding = RepositoryBinding::new("github", "owner/repo", "abc123", true).unwrap();
        let submission = Submission::new(
            EntityId("submission-1".into()),
            EntityId("team-1".into()),
            EntityId("hack-1".into()),
            summary,
            binding.clone(),
            SystemTime::now(),
        )
        .unwrap();

        let extracted = submission.repository_binding();
        assert_eq!(extracted, &binding);
        assert!(extracted.is_private());
    }
}
