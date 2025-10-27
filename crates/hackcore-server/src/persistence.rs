use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use core_domain::{hackathon::Hackathon, submission::Submission, team::Team, user::User};
use persistence::{
    migrations,
    repositories::{
        hackathons::{HackathonRecord, HackathonRepository},
        submissions::SubmissionRepository,
        teams::TeamRepository,
        users::{UserRecord, UserRepository},
    },
    Database,
};
use sqlx::PgPool;

#[async_trait]
pub trait PersistenceGateway: Send + Sync {
    async fn get_user(&self, id: &str) -> Result<Option<User>>;
    async fn list_users(&self, limit: usize, offset: usize) -> Result<Vec<User>>;
    async fn get_hackathon(&self, id: &str) -> Result<Option<Hackathon>>;
    async fn list_hackathons(&self, limit: usize, offset: usize) -> Result<Vec<Hackathon>>;
    async fn insert_submission(&self, submission: Submission) -> Result<Submission>;
    async fn get_submission(&self, id: &str) -> Result<Option<Submission>>;
    async fn get_team(&self, id: &str) -> Result<Option<Team>>;
}

#[derive(Clone)]
pub struct PostgresPersistence {
    database: Database,
}

impl PostgresPersistence {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub async fn run_migrations(&self) -> Result<()> {
        migrations::run(self.database.pool()).await?;
        Ok(())
    }
}

#[async_trait]
impl PersistenceGateway for PostgresPersistence {
    async fn get_user(&self, id: &str) -> Result<Option<User>> {
        let repo = UserRepository::new(self.database.pool());
        repo.find_by_id(id).await.map_err(Into::into)
    }

    async fn list_users(&self, limit: usize, offset: usize) -> Result<Vec<User>> {
        let records = sqlx::query_as::<_, UserRecord>(
            "SELECT id, display_name, email FROM users ORDER BY id LIMIT $1 OFFSET $2",
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(self.database.pool())
        .await?;

        records
            .into_iter()
            .map(User::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    async fn get_hackathon(&self, id: &str) -> Result<Option<Hackathon>> {
        let repo = HackathonRepository::new(self.database.pool());
        repo.find_by_id(id).await.map_err(Into::into)
    }

    async fn list_hackathons(&self, limit: usize, offset: usize) -> Result<Vec<Hackathon>> {
        let records = sqlx::query_as::<_, HackathonRecord>(
            r#"SELECT id, name, registration_deadline, submission_deadline, team_size_limit
            FROM hackathons ORDER BY registration_deadline DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(self.database.pool())
        .await?;

        records
            .into_iter()
            .map(Hackathon::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    async fn insert_submission(&self, submission: Submission) -> Result<Submission> {
        let repo = SubmissionRepository::new(self.database.pool());
        repo.insert(&submission).await?;
        Ok(submission)
    }

    async fn get_submission(&self, id: &str) -> Result<Option<Submission>> {
        let repo = SubmissionRepository::new(self.database.pool());
        repo.find_by_id(id).await.map_err(Into::into)
    }

    async fn get_team(&self, id: &str) -> Result<Option<Team>> {
        let repo = TeamRepository::new(self.database.pool());
        repo.find_by_id(id).await.map_err(Into::into)
    }
}

impl From<PostgresPersistence> for Database {
    fn from(value: PostgresPersistence) -> Self {
        value.database
    }
}

impl PostgresPersistence {
    pub async fn new_with_pool(pool: PgPool) -> Self {
        let database = Database::new(pool);
        Self { database }
    }
}

pub async fn connect_pool(settings: &crate::config::Settings) -> Result<PgPool> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(settings.database_max_connections)
        .connect(&settings.database_url)
        .await?;
    Ok(pool)
}

pub fn default_shutdown_signal() -> impl std::future::Future<Output = ()> {
    async {
        let _ = tokio::signal::ctrl_c().await;
    }
}

pub type SharedPersistence<P> = Arc<P>;
