use anyhow::Result;
use core_domain::{
    model::{AggregateRoot, EntityId},
    user::{DisplayName, EmailAddress, User, UserProfile},
};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, FromRow)]
pub struct UserRecord {
    pub id: String,
    pub display_name: String,
    pub email: String,
}

impl From<&User> for UserRecord {
    fn from(user: &User) -> Self {
        Self {
            id: user.id().to_owned(),
            display_name: user.profile().display_name().value().to_owned(),
            email: user.profile().email().value().to_owned(),
        }
    }
}

impl TryFrom<UserRecord> for User {
    type Error = anyhow::Error;

    fn try_from(value: UserRecord) -> Result<Self> {
        let display_name = DisplayName::new(value.display_name)?;
        let email = EmailAddress::new(value.email)?;
        let profile = UserProfile::new(display_name, email);
        User::new(EntityId(value.id), profile)
    }
}

/// Data access facade for user aggregates.
#[derive(Debug, Clone)]
pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn upsert(&self, user: &User) -> Result<()> {
        let record = UserRecord::from(user);
        sqlx::query(
            r#"INSERT INTO users (id, display_name, email)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE
            SET display_name = EXCLUDED.display_name,
                email = EXCLUDED.email"#,
        )
        .bind(&record.id)
        .bind(&record.display_name)
        .bind(&record.email)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>> {
        let record = sqlx::query_as::<_, UserRecord>(
            "SELECT id, display_name, email FROM users WHERE id = $1",
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
    use sqlx::PgPool;

    #[sqlx::test(migrations = "./migrations")]
    async fn upsert_and_fetch_user(pool: PgPool) -> sqlx::Result<()> {
        let repo = UserRepository::new(&pool);
        let profile = UserProfile::new(
            DisplayName::new("Alice").unwrap(),
            EmailAddress::new("alice@example.com").unwrap(),
        );
        let user = User::new(EntityId("user-1".into()), profile).unwrap();

        repo.upsert(&user)
            .await
            .expect("user upsert should succeed");
        let loaded = repo
            .find_by_id("user-1")
            .await
            .expect("query should succeed")
            .expect("user should be present");

        assert_eq!(loaded.profile().email().value(), "alice@example.com");
        Ok(())
    }
}
