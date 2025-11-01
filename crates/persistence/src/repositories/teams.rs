use anyhow::Result;
use core_domain::{
    model::{AggregateRoot, EntityId},
    team::{Team, TeamName},
};
use sqlx::{PgPool, Row};

#[derive(Debug, Clone)]
pub struct TeamRecord {
    pub id: String,
    pub hackathon_id: String,
    pub name: String,
    pub members: Vec<String>,
}

impl From<&Team> for TeamRecord {
    fn from(team: &Team) -> Self {
        let members = team
            .members()
            .iter()
            .map(|member| member.0.clone())
            .collect();

        Self {
            id: team.id().to_owned(),
            hackathon_id: team.hackathon_id().0.clone(),
            name: team.name().value().to_owned(),
            members,
        }
    }
}

impl TryFrom<TeamRecord> for Team {
    type Error = anyhow::Error;

    fn try_from(value: TeamRecord) -> Result<Self> {
        let name = TeamName::new(value.name)?;
        let members = value.members.into_iter().map(EntityId).collect::<Vec<_>>();
        Team::new(
            EntityId(value.id),
            EntityId(value.hackathon_id),
            name,
            members,
        )
    }
}

/// Data access facade for team aggregates.
#[derive(Debug, Clone)]
pub struct TeamRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> TeamRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn upsert(&self, team: &Team) -> Result<()> {
        let record = TeamRecord::from(team);
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            r#"INSERT INTO teams (id, hackathon_id, name)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE SET
                hackathon_id = EXCLUDED.hackathon_id,
                name = EXCLUDED.name"#,
        )
        .bind(&record.id)
        .bind(&record.hackathon_id)
        .bind(&record.name)
        .execute(&mut *tx)
        .await?;

        sqlx::query("DELETE FROM team_members WHERE team_id = $1")
            .bind(&record.id)
            .execute(&mut *tx)
            .await?;

        for member in &record.members {
            sqlx::query("INSERT INTO team_members (team_id, user_id) VALUES ($1, $2)")
                .bind(&record.id)
                .bind(member)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Team>> {
        let row = sqlx::query("SELECT id, hackathon_id, name FROM teams WHERE id = $1")
            .bind(id)
            .fetch_optional(self.pool)
            .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let team_id: String = row.get("id");
        let members = sqlx::query_scalar(
            "SELECT user_id FROM team_members WHERE team_id = $1 ORDER BY user_id",
        )
        .bind(&team_id)
        .fetch_all(self.pool)
        .await?;

        let record = TeamRecord {
            id: team_id,
            hackathon_id: row.get("hackathon_id"),
            name: row.get("name"),
            members,
        };

        Ok(Some(record.try_into()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::hackathons::HackathonRepository;
    use core_domain::{
        hackathon::{Hackathon, HackathonName, TeamSizeLimit},
        team::TeamName,
    };
    use sqlx::PgPool;
    use std::time::{Duration, SystemTime};

    #[sqlx::test(migrations = "./migrations")]
    async fn upsert_and_fetch_team(pool: PgPool) -> sqlx::Result<()> {
        let repo = TeamRepository::new(&pool);
        let hackathon_repo = HackathonRepository::new(&pool);

        let now = SystemTime::now();
        let hackathon = Hackathon::new(
            EntityId("hack-1".into()),
            HackathonName::new("Hack").unwrap(),
            now,
            now + Duration::from_secs(3600),
            TeamSizeLimit::new(4).unwrap(),
        )
        .unwrap();
        hackathon_repo.upsert(&hackathon).await.unwrap();

        let members = vec![EntityId("user-1".into()), EntityId("user-2".into())];
        let team = Team::new(
            EntityId("team-1".into()),
            EntityId("hack-1".into()),
            TeamName::new("Dream Team").unwrap(),
            members,
        )
        .unwrap();

        repo.upsert(&team)
            .await
            .expect("team upsert should succeed");
        let loaded = repo
            .find_by_id("team-1")
            .await
            .expect("query should succeed")
            .expect("team should be present");

        assert_eq!(loaded.members().len(), 2);
        assert_eq!(loaded.hackathon_id().0, "hack-1");
        Ok(())
    }
}
