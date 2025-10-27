use anyhow::Result;

use crate::{
    model::{AggregateRoot, EntityId},
    validate_invariant,
};

/// Aggregate representing a participant of the platform.
#[derive(Debug, Clone)]
pub struct User {
    id: EntityId,
    profile: UserProfile,
}

impl User {
    pub fn new(id: EntityId, profile: UserProfile) -> Result<Self> {
        validate_invariant(!profile.display_name.value().is_empty())?;
        Ok(Self { id, profile })
    }

    pub fn profile(&self) -> &UserProfile {
        &self.profile
    }
}

impl AggregateRoot for User {
    fn id(&self) -> &str {
        &self.id.0
    }
}

/// Value object holding the public profile of a user.
#[derive(Debug, Clone)]
pub struct UserProfile {
    display_name: DisplayName,
    email: EmailAddress,
}

impl UserProfile {
    pub fn new(display_name: DisplayName, email: EmailAddress) -> Self {
        Self {
            display_name,
            email,
        }
    }

    pub fn display_name(&self) -> &DisplayName {
        &self.display_name
    }

    pub fn email(&self) -> &EmailAddress {
        &self.email
    }
}

/// Public name shown within hackathons.
#[derive(Debug, Clone)]
pub struct DisplayName(String);

impl DisplayName {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        validate_invariant(!value.trim().is_empty())?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Email address used for notifications.
#[derive(Debug, Clone)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        validate_invariant(value.contains('@') && value.contains('.'))?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_name_must_not_be_empty() {
        assert!(DisplayName::new("").is_err());
        assert!(DisplayName::new("  ").is_err());
    }

    #[test]
    fn email_address_requires_at_symbol() {
        assert!(EmailAddress::new("invalid").is_err());
        assert!(EmailAddress::new("user@example.com").is_ok());
    }

    #[test]
    fn user_creation_requires_valid_profile() {
        let profile = UserProfile::new(
            DisplayName::new("Alice").unwrap(),
            EmailAddress::new("alice@example.com").unwrap(),
        );
        let user = User::new(EntityId("user-1".into()), profile.clone()).unwrap();
        assert_eq!(user.profile().display_name().value(), "Alice");
    }
}
