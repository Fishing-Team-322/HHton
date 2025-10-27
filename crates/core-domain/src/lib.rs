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
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct EntityId(pub String);
}

/// Domain services encapsulate operations that coordinate aggregates.
pub mod services {
    use super::model::{AggregateRoot, EntityId};

    /// Provides shared utilities for aggregate management.
    pub struct AggregateService;

    impl AggregateService {
        /// Ensures the aggregate root has a valid identifier assigned.
        pub fn ensure_identity<T: AggregateRoot>(aggregate: &T) -> EntityId {
            EntityId(aggregate.id().to_owned())
        }
    }
}

/// Validates invariants defined by the domain experts.
pub fn validate_invariant(predicate: bool) -> Result<()> {
    anyhow::ensure!(predicate, "domain invariant violated");
    Ok(())
}
