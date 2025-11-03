//! Core domain primitives shared across bounded contexts.

use anyhow::Result;

/// Models that define the heart of the business logic.
pub mod model {
    /// Marker trait for aggregate roots within the platform.
    pub trait AggregateRoot {
        /// Returns the stable identifier of the aggregate root instance.
        fn id(&self) -> &str;
    }

    /// A lightweight identifier type used across aggregates.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct EntityId(pub String);
}

pub mod application;
pub mod hackathon;
pub mod rating;
pub mod submission;
pub mod team;
pub mod user;

/// Domain services encapsulate operations that coordinate aggregates.
pub mod services {
    use std::time::SystemTime;

    use anyhow::Result;

    use super::model::{AggregateRoot, EntityId};
    use crate::{
        hackathon::{Hackathon, TeamSizeLimit},
        team::Team,
        validate_invariant,
    };

    /// Provides shared utilities for aggregate management.
    pub struct AggregateService;

    impl AggregateService {
        /// Ensures the aggregate root has a valid identifier assigned.
        pub fn ensure_identity<T: AggregateRoot>(aggregate: &T) -> EntityId {
            EntityId(aggregate.id().to_owned())
        }
    }

    /// Domain rules ensuring hackathon-related invariants.
    pub struct HackathonRules;

    impl HackathonRules {
        pub fn ensure_registration_open(hackathon: &Hackathon, now: SystemTime) -> Result<()> {
            validate_invariant(now <= hackathon.registration_deadline())
        }

        pub fn ensure_submission_open(hackathon: &Hackathon, now: SystemTime) -> Result<()> {
            validate_invariant(now <= hackathon.submission_deadline())
        }

        pub fn ensure_team_size_limit(limit: &TeamSizeLimit, member_count: usize) -> Result<()> {
            validate_invariant(limit.allows(member_count))
        }
    }

    /// Domain rules ensuring team-related invariants.
    pub struct TeamRules;

    impl TeamRules {
        pub fn ensure_can_join(team: &Team, member: &EntityId) -> Result<()> {
            validate_invariant(!team.has_member(member))
        }

        pub fn ensure_within_limit(hackathon: &Hackathon, team: &Team) -> Result<()> {
            HackathonRules::ensure_team_size_limit(hackathon.team_size_limit(), team.member_count())
        }

        pub fn ensure_room_for_new_member(hackathon: &Hackathon, team: &Team) -> Result<()> {
            let prospective = team.member_count().saturating_add(1);
            HackathonRules::ensure_team_size_limit(hackathon.team_size_limit(), prospective)
        }
    }
}

/// Validates invariants defined by the domain experts.
pub fn validate_invariant(predicate: bool) -> Result<()> {
    anyhow::ensure!(predicate, "domain invariant violated");
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use super::{
        hackathon::{Hackathon, HackathonDescription, HackathonName, TeamSizeLimit},
        model::EntityId,
        services::{HackathonRules, TeamRules},
        team::{Team, TeamName},
    };

    #[test]
    fn ensure_registration_deadline_is_enforced() {
        let now = SystemTime::now();
        let hackathon = Hackathon::new(
            EntityId("hack-1".into()),
            HackathonName::new("Hack").unwrap(),
            HackathonDescription::new("Hack description").unwrap(),
            now + Duration::from_secs(60),
            now + Duration::from_secs(90),
            now + Duration::from_secs(100),
            now + Duration::from_secs(120),
            TeamSizeLimit::new(3).unwrap(),
        )
        .unwrap();

        assert!(HackathonRules::ensure_registration_open(&hackathon, now).is_ok());
        assert!(HackathonRules::ensure_registration_open(
            &hackathon,
            now + Duration::from_secs(61)
        )
        .is_err());
    }

    #[test]
    fn ensure_submission_deadline_is_enforced() {
        let now = SystemTime::now();
        let hackathon = Hackathon::new(
            EntityId("hack-1".into()),
            HackathonName::new("Hack").unwrap(),
            HackathonDescription::new("Hack description").unwrap(),
            now - Duration::from_secs(60),
            now - Duration::from_secs(30),
            now,
            now + Duration::from_secs(30),
            TeamSizeLimit::new(3).unwrap(),
        )
        .unwrap();

        assert!(HackathonRules::ensure_submission_open(&hackathon, now).is_ok());
        assert!(
            HackathonRules::ensure_submission_open(&hackathon, now + Duration::from_secs(31))
                .is_err()
        );
    }

    #[test]
    fn ensure_member_cannot_join_twice() {
        let team = Team::new(
            EntityId("team-1".into()),
            EntityId("hack-1".into()),
            TeamName::new("Team").unwrap(),
            vec![EntityId("user-1".into())],
        )
        .unwrap();
        assert!(TeamRules::ensure_can_join(&team, &EntityId("user-1".into())).is_err());
    }

    #[test]
    fn ensure_team_member_limit_is_checked() {
        let now = SystemTime::now();
        let hackathon = Hackathon::new(
            EntityId("hack-1".into()),
            HackathonName::new("Hack").unwrap(),
            HackathonDescription::new("Hack description").unwrap(),
            now,
            now + Duration::from_secs(30),
            now + Duration::from_secs(60),
            now + Duration::from_secs(120),
            TeamSizeLimit::new(2).unwrap(),
        )
        .unwrap();

        let team = Team::new(
            EntityId("team-1".into()),
            EntityId("hack-1".into()),
            TeamName::new("Team").unwrap(),
            vec![EntityId("user-1".into()), EntityId("user-2".into())],
        )
        .unwrap();

        assert!(TeamRules::ensure_within_limit(&hackathon, &team).is_ok());
        assert!(TeamRules::ensure_room_for_new_member(&hackathon, &team).is_err());

        let roomy_hackathon = Hackathon::new(
            EntityId("hack-2".into()),
            HackathonName::new("Hack").unwrap(),
            HackathonDescription::new("Hack description").unwrap(),
            now,
            now + Duration::from_secs(30),
            now + Duration::from_secs(60),
            now + Duration::from_secs(120),
            TeamSizeLimit::new(3).unwrap(),
        )
        .unwrap();
        assert!(TeamRules::ensure_room_for_new_member(&roomy_hackathon, &team).is_ok());
        let mut overfull_team = team.clone();
        overfull_team.add_member(EntityId("user-3".into())).unwrap();
        assert!(TeamRules::ensure_within_limit(&hackathon, &overfull_team).is_err());
    }
}
