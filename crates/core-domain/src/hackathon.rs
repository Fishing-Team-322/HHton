use std::time::SystemTime;

use anyhow::Result;

use crate::{
    model::{AggregateRoot, EntityId},
    validate_invariant,
};

/// Aggregate representing a hackathon event.
#[derive(Debug, Clone)]
pub struct Hackathon {
    id: EntityId,
    name: HackathonName,
    registration_deadline: SystemTime,
    submission_deadline: SystemTime,
    team_size_limit: TeamSizeLimit,
}

impl Hackathon {
    pub fn new(
        id: EntityId,
        name: HackathonName,
        registration_deadline: SystemTime,
        submission_deadline: SystemTime,
        team_size_limit: TeamSizeLimit,
    ) -> Result<Self> {
        validate_invariant(registration_deadline <= submission_deadline)?;
        Ok(Self {
            id,
            name,
            registration_deadline,
            submission_deadline,
            team_size_limit,
        })
    }

    pub fn name(&self) -> &HackathonName {
        &self.name
    }

    pub fn registration_deadline(&self) -> SystemTime {
        self.registration_deadline
    }

    pub fn submission_deadline(&self) -> SystemTime {
        self.submission_deadline
    }

    pub fn team_size_limit(&self) -> &TeamSizeLimit {
        &self.team_size_limit
    }
}

impl AggregateRoot for Hackathon {
    fn id(&self) -> &str {
        &self.id.0
    }
}

/// Value object capturing the hackathon display name.
#[derive(Debug, Clone)]
pub struct HackathonName(String);

impl HackathonName {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        validate_invariant(!value.trim().is_empty())?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Value object describing the team size limit configured by organizers.
#[derive(Debug, Clone)]
pub struct TeamSizeLimit(u32);

impl TeamSizeLimit {
    pub fn new(limit: u32) -> Result<Self> {
        validate_invariant(limit > 0)?;
        Ok(Self(limit))
    }

    pub fn max_members(&self) -> u32 {
        self.0
    }

    pub fn allows(&self, members: usize) -> bool {
        members as u32 <= self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> EntityId {
        EntityId(value.to_string())
    }

    #[test]
    fn deadlines_are_ordered() {
        let now = SystemTime::now();
        let later = now + std::time::Duration::from_secs(60);
        let name = HackathonName::new("Hack 2024").unwrap();
        let limit = TeamSizeLimit::new(4).unwrap();
        let hackathon = Hackathon::new(id("hack-1"), name, now, later, limit);
        assert!(hackathon.is_ok());
    }

    #[test]
    fn registration_deadline_must_precede_submission() {
        let now = SystemTime::now();
        let name = HackathonName::new("Hack 2024").unwrap();
        let limit = TeamSizeLimit::new(4).unwrap();
        let hackathon = Hackathon::new(
            id("hack-1"),
            name,
            now,
            now - std::time::Duration::from_secs(60),
            limit,
        );
        assert!(hackathon.is_err());
    }

    #[test]
    fn team_limit_must_be_positive() {
        assert!(TeamSizeLimit::new(0).is_err());
    }
}
