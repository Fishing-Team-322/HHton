use std::time::SystemTime;

use anyhow::Result;
use chrono::{DateTime, Utc};
use core_domain::{
    model::{AggregateRoot, EntityId},
    submission::{RepositoryBinding, Submission, SubmissionSummary},
};
use serde_json::Value;
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, FromRow)]
pub struct SubmissionRecord {
    pub id: String,
    pub team_id: String,
    pub hackathon_id: String,
    pub summary: String,
    pub provider: Option<String>,
    pub repository: Option<String>,
    pub commit_sha: Option<String>,
    pub is_private: bool,
    pub proof_status: Option<String>,
    pub metadata_json: Option<Value>,
    pub created_at: DateTime<Utc>,
}

impl From<&Submission> for SubmissionRecord {
    fn from(submission: &Submission) -> Self {
        let repository_binding = submission.repository_binding();
        Self {
            id: submission.id().to_owned(),
            team_id: submission.team_id().0.clone(),
            hackathon_id: submission.hackathon_id().0.clone(),
            summary: submission.summary().value().to_owned(),
            provider: repository_binding.map(|binding| binding.provider().to_owned()),
            repository: repository_binding.map(|binding| binding.repository().to_owned()),
            commit_sha: repository_binding.map(|binding| binding.reference().to_owned()),
            is_private: repository_binding
                .map(|binding| binding.is_private())
                .unwrap_or(false),
            proof_status: None,
            metadata_json: None,
            created_at: DateTime::<Utc>::from(submission.created_at()),
        }
    }
}

impl TryFrom<SubmissionRecord> for Submission {
    type Error = anyhow::Error;

    fn try_from(value: SubmissionRecord) -> Result<Self> {
        let summary = SubmissionSummary::new(value.summary)?;
        let repository_binding =
            match (value.provider, value.repository, value.commit_sha) {
                (Some(provider), Some(repository), Some(commit_sha)) => Some(
                    RepositoryBinding::new(provider, repository, commit_sha, value.is_private)?,
                ),
                _ => None,
            };
        Submission::new(
            EntityId(value.id),
            EntityId(value.team_id),
            EntityId(value.hackathon_id),
            summary,
            repository_binding,
            SystemTime::from(value.created_at),
        )
    }
}

/// Data access facade for submission aggregates.
#[derive(Debug, Clone)]
pub struct SubmissionRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> SubmissionRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, submission: &Submission) -> Result<()> {
        let record = SubmissionRecord::from(submission);
        sqlx::query(
            r#"INSERT INTO submissions (
                id,
                team_id,
                hackathon_id,
                summary,
                provider,
                repository,
                commit_sha,
                is_private,
                proof_status,
                metadata_json,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                team_id = EXCLUDED.team_id,
                hackathon_id = EXCLUDED.hackathon_id,
                summary = EXCLUDED.summary,
                provider = EXCLUDED.provider,
                repository = EXCLUDED.repository,
                commit_sha = EXCLUDED.commit_sha,
                is_private = EXCLUDED.is_private,
                proof_status = EXCLUDED.proof_status,
                metadata_json = EXCLUDED.metadata_json,
                created_at = EXCLUDED.created_at"#,
        )
        .bind(&record.id)
        .bind(&record.team_id)
        .bind(&record.hackathon_id)
        .bind(&record.summary)
        .bind(&record.provider)
        .bind(&record.repository)
        .bind(&record.commit_sha)
        .bind(record.is_private)
        .bind(&record.proof_status)
        .bind(&record.metadata_json)
        .bind(record.created_at)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Submission>> {
        let record = sqlx::query_as::<_, SubmissionRecord>(
            "SELECT id, team_id, hackathon_id, summary, provider, repository, commit_sha, is_private, proof_status, metadata_json, created_at FROM submissions WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;

        match record {
            Some(row) => Ok(Some(row.try_into()?)),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{
        hackathons::HackathonRepository, teams::TeamRepository, users::UserRepository,
    };
    use core_domain::{
        hackathon::{Hackathon, HackathonName, TeamSizeLimit},
        team::TeamName,
        user::{DisplayName, EmailAddress, User, UserProfile},
    };
    use sqlx::PgPool;
    use std::time::{Duration, SystemTime};

    #[sqlx::test(migrations = "./migrations")]
    async fn insert_and_fetch_submission(pool: PgPool) -> sqlx::Result<()> {
        let user_repo = UserRepository::new(&pool);
        let team_repo = TeamRepository::new(&pool);
        let hackathon_repo = HackathonRepository::new(&pool);
        let submission_repo = SubmissionRepository::new(&pool);

        let profile = UserProfile::new(
            DisplayName::new("Alice").unwrap(),
            EmailAddress::new("alice@example.com").unwrap(),
        );
        let user = User::new(EntityId("user-1".into()), profile).unwrap();
        user_repo.upsert(&user).await.unwrap();

        let team = core_domain::team::Team::new(
            EntityId("team-1".into()),
            TeamName::new("Dream Team").unwrap(),
            vec![EntityId("user-1".into())],
        )
        .unwrap();
        team_repo.upsert(&team).await.unwrap();

        let now = SystemTime::now();
        let hackathon = Hackathon::new(
            EntityId("hack-1".into()),
            HackathonName::new("Hack").unwrap(),
            now - Duration::from_secs(3600),
            now + Duration::from_secs(3600),
            TeamSizeLimit::new(4).unwrap(),
        )
        .unwrap();
        hackathon_repo.upsert(&hackathon).await.unwrap();

        let submission = Submission::new(
            EntityId("submission-1".into()),
            EntityId("team-1".into()),
            EntityId("hack-1".into()),
            SubmissionSummary::new("Great project").unwrap(),
            Some(
                RepositoryBinding::new("github", "owner/repo", "abc123", true)
                    .expect("binding should be valid"),
            ),
            SystemTime::now(),
        )
        .unwrap();

        submission_repo
            .insert(&submission)
            .await
            .expect("submission insert should succeed");
        let loaded = submission_repo
            .find_by_id("submission-1")
            .await
            .expect("query should succeed")
            .expect("submission should be present");

        assert_eq!(loaded.summary().value(), "Great project");
        let binding = loaded
            .repository_binding()
            .expect("binding should roundtrip");
        assert_eq!(binding.provider(), "github");
        assert_eq!(binding.repository(), "owner/repo");
        assert_eq!(binding.reference(), "abc123");
        assert!(binding.is_private());
        Ok(())
    }
}
