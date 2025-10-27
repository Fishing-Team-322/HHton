use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use chrono::{DateTime, Utc};
use contracts::events::Event;
use core_domain::{
    hackathon::{Hackathon, HackathonName, TeamSizeLimit},
    model::{AggregateRoot, EntityId},
};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, FromRow)]
pub struct HackathonRecord {
    pub id: String,
    pub name: String,
    pub registration_deadline: DateTime<Utc>,
    pub submission_deadline: DateTime<Utc>,
    pub team_size_limit: i32,
}

impl From<&Hackathon> for HackathonRecord {
    fn from(hackathon: &Hackathon) -> Self {
        Self {
            id: hackathon.id().to_owned(),
            name: hackathon.name().value().to_owned(),
            registration_deadline: DateTime::<Utc>::from(hackathon.registration_deadline()),
            submission_deadline: DateTime::<Utc>::from(hackathon.submission_deadline()),
            team_size_limit: hackathon.team_size_limit().max_members() as i32,
        }
    }
}

impl TryFrom<HackathonRecord> for Hackathon {
    type Error = anyhow::Error;

    fn try_from(value: HackathonRecord) -> Result<Self> {
        let name = HackathonName::new(value.name)?;
        let registration_deadline: SystemTime = value.registration_deadline.into();
        let submission_deadline: SystemTime = value.submission_deadline.into();
        let team_size_limit = TeamSizeLimit::new(u32::try_from(value.team_size_limit)?)?;

        Hackathon::new(
            EntityId(value.id),
            name,
            registration_deadline,
            submission_deadline,
            team_size_limit,
        )
    }
}

/// Data access facade for hackathon aggregates.
#[derive(Debug, Clone)]
pub struct HackathonRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> HackathonRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn upsert(&self, hackathon: &Hackathon) -> Result<()> {
        let record = HackathonRecord::from(hackathon);
        sqlx::query(
            r#"INSERT INTO hackathons (
                id,
                name,
                registration_deadline,
                submission_deadline,
                team_size_limit
            ) VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                registration_deadline = EXCLUDED.registration_deadline,
                submission_deadline = EXCLUDED.submission_deadline,
                team_size_limit = EXCLUDED.team_size_limit"#,
        )
        .bind(&record.id)
        .bind(&record.name)
        .bind(record.registration_deadline)
        .bind(record.submission_deadline)
        .bind(record.team_size_limit)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Hackathon>> {
        let record = sqlx::query_as::<_, HackathonRecord>(
            r#"SELECT id, name, registration_deadline, submission_deadline, team_size_limit
            FROM hackathons WHERE id = $1"#,
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

/// Convert a domain [`Hackathon`] aggregate into the public gRPC DTO.
pub fn hackathon_to_contract_event(hackathon: &Hackathon) -> Event {
    Event {
        id: hackathon.id().to_owned(),
        name: hackathon.name().value().to_owned(),
        description: String::new(),
        start_time: 0,
        end_time: 0,
        registration_deadline: system_time_to_epoch_seconds(hackathon.registration_deadline()),
        submission_deadline: system_time_to_epoch_seconds(hackathon.submission_deadline()),
        team_size_limit: hackathon.team_size_limit().max_members(),
    }
}

fn system_time_to_epoch_seconds(time: SystemTime) -> i64 {
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as i64,
        Err(error) => -(error.duration().as_secs() as i64),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    #[sqlx::test(migrations = "./migrations")]
    async fn upsert_and_fetch_hackathon(pool: PgPool) -> sqlx::Result<()> {
        let repo = HackathonRepository::new(&pool);
        let now = SystemTime::now();
        let hackathon = Hackathon::new(
            EntityId("hack-1".into()),
            HackathonName::new("Hack").unwrap(),
            now,
            now + Duration::from_secs(3600),
            TeamSizeLimit::new(4).unwrap(),
        )
        .unwrap();

        repo.upsert(&hackathon)
            .await
            .expect("hackathon upsert should succeed");
        let loaded = repo
            .find_by_id("hack-1")
            .await
            .expect("query should succeed")
            .expect("hackathon should be present");

        assert_eq!(loaded.team_size_limit().max_members(), 4);
        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn hackathon_is_fully_translated_to_grpc(pool: PgPool) -> sqlx::Result<()> {
        let repo = HackathonRepository::new(&pool);
        let registration_deadline = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let submission_deadline = UNIX_EPOCH + Duration::from_secs(1_700_086_400);
        let hackathon = Hackathon::new(
            EntityId("hack-dto".into()),
            HackathonName::new("Hack DTO").unwrap(),
            registration_deadline,
            submission_deadline,
            TeamSizeLimit::new(6).unwrap(),
        )
        .unwrap();

        repo.upsert(&hackathon)
            .await
            .expect("hackathon upsert should succeed");

        let loaded = repo
            .find_by_id("hack-dto")
            .await
            .expect("query should succeed")
            .expect("hackathon should be present");

        let dto = hackathon_to_contract_event(&loaded);

        assert_eq!(dto.id, "hack-dto");
        assert_eq!(dto.name, "Hack DTO");
        assert_eq!(
            dto.registration_deadline,
            system_time_to_epoch_seconds(registration_deadline)
        );
        assert_eq!(
            dto.submission_deadline,
            system_time_to_epoch_seconds(submission_deadline)
        );
        assert_eq!(dto.team_size_limit, 6);
        Ok(())
    }
}
