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
    description: HackathonDescription,
    registration_deadline: SystemTime,
    start_time: SystemTime,
    end_time: SystemTime,
    submission_deadline: SystemTime,
    team_size_limit: TeamSizeLimit,
}

impl Hackathon {
    pub fn new(
        id: EntityId,
        name: HackathonName,
        description: HackathonDescription,
        registration_deadline: SystemTime,
        start_time: SystemTime,
        end_time: SystemTime,
        submission_deadline: SystemTime,
        team_size_limit: TeamSizeLimit,
    ) -> Result<Self> {
        validate_invariant(registration_deadline <= start_time)?;
        validate_invariant(start_time <= end_time)?;
        validate_invariant(end_time <= submission_deadline)?;
        Ok(Self {
            id,
            name,
            description,
            registration_deadline,
            start_time,
            end_time,
            submission_deadline,
            team_size_limit,
        })
    }

    pub fn name(&self) -> &HackathonName {
        &self.name
    }

    pub fn description(&self) -> &HackathonDescription {
        &self.description
    }

    pub fn registration_deadline(&self) -> SystemTime {
        self.registration_deadline
    }

    pub fn start_time(&self) -> SystemTime {
        self.start_time
    }

    pub fn end_time(&self) -> SystemTime {
        self.end_time
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

/// Value object capturing the hackathon description.
#[derive(Debug, Clone)]
pub struct HackathonDescription(String);

impl HackathonDescription {
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
        let description = HackathonDescription::new("The best hackathon").unwrap();
        let limit = TeamSizeLimit::new(4).unwrap();
        let hackathon = Hackathon::new(
            id("hack-1"),
            name,
            description,
            now,
            now,
            later,
            later,
            limit,
        );
        assert!(hackathon.is_ok());
    }

    #[test]
    fn start_time_must_precede_end_time() {
        let now = SystemTime::now();
        let name = HackathonName::new("Hack 2024").unwrap();
        let description = HackathonDescription::new("Another hackathon").unwrap();
        let limit = TeamSizeLimit::new(4).unwrap();
        let hackathon = Hackathon::new(
            id("hack-1"),
            name,
            description,
            now,
            now + std::time::Duration::from_secs(120),
            now + std::time::Duration::from_secs(60),
            now + std::time::Duration::from_secs(360),
            limit,
        );
        assert!(hackathon.is_err());
    }

    #[test]
    fn end_time_must_precede_submission_deadline() {
        let now = SystemTime::now();
        let name = HackathonName::new("Hack 2024").unwrap();
        let description = HackathonDescription::new("The best hackathon").unwrap();
        let limit = TeamSizeLimit::new(4).unwrap();
        let hackathon = Hackathon::new(
            id("hack-1"),
            name,
            description,
            now,
            now + std::time::Duration::from_secs(60),
            now + std::time::Duration::from_secs(120),
            now + std::time::Duration::from_secs(90),
            limit,
        );
        assert!(hackathon.is_err());
    }

    #[test]
    fn registration_deadline_must_precede_start_time() {
        let now = SystemTime::now();
        let name = HackathonName::new("Hack 2024").unwrap();
        let description = HackathonDescription::new("Another hackathon").unwrap();
        let limit = TeamSizeLimit::new(4).unwrap();
        let hackathon = Hackathon::new(
            id("hack-1"),
            name,
            description,
            now + std::time::Duration::from_secs(120),
            now,
            now + std::time::Duration::from_secs(60),
            now + std::time::Duration::from_secs(360),
            limit,
        );
        assert!(hackathon.is_err());
    }

    #[test]
    fn description_must_not_be_blank() {
        assert!(HackathonDescription::new("   ").is_err());
    }

    #[test]
    fn team_limit_must_be_positive() {
        assert!(TeamSizeLimit::new(0).is_err());
    }
}
