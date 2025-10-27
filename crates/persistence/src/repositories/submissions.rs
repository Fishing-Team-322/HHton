use std::time::SystemTime;

use anyhow::Result;
use chrono::{DateTime, Utc};
use core_domain::{
    model::{AggregateRoot, EntityId},
    submission::{Submission, SubmissionSummary},
};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, FromRow)]
pub struct SubmissionRecord {
    pub id: String,
    pub team_id: String,
    pub hackathon_id: String,
    pub summary: String,
    pub created_at: DateTime<Utc>,
}

impl From<&Submission> for SubmissionRecord {
    fn from(submission: &Submission) -> Self {
        Self {
            id: submission.id().to_owned(),
            team_id: submission.team_id().0.clone(),
            hackathon_id: submission.hackathon_id().0.clone(),
            summary: submission.summary().value().to_owned(),
            created_at: DateTime::<Utc>::from(submission.created_at()),
        }
    }
}

impl TryFrom<SubmissionRecord> for Submission {
    type Error = anyhow::Error;

    fn try_from(value: SubmissionRecord) -> Result<Self> {
        let summary = SubmissionSummary::new(value.summary)?;
        Submission::new(
            EntityId(value.id),
            EntityId(value.team_id),
            EntityId(value.hackathon_id),
            summary,
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
            r#"INSERT INTO submissions (id, team_id, hackathon_id, summary, created_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                team_id = EXCLUDED.team_id,
                hackathon_id = EXCLUDED.hackathon_id,
                summary = EXCLUDED.summary,
                created_at = EXCLUDED.created_at"#,
        )
        .bind(&record.id)
        .bind(&record.team_id)
        .bind(&record.hackathon_id)
        .bind(&record.summary)
        .bind(record.created_at)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Submission>> {
        let record = sqlx::query_as::<_, SubmissionRecord>(
            "SELECT id, team_id, hackathon_id, summary, created_at FROM submissions WHERE id = $1",
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
        Ok(())
    }
}
