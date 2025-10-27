use std::collections::HashSet;

use anyhow::Result;

use crate::{
    model::{AggregateRoot, EntityId},
    validate_invariant,
};

/// Aggregate representing a team participating in a hackathon.
#[derive(Debug, Clone)]
pub struct Team {
    id: EntityId,
    name: TeamName,
    members: Vec<EntityId>,
}

impl Team {
    pub fn new(id: EntityId, name: TeamName, members: Vec<EntityId>) -> Result<Self> {
        validate_invariant(!members.is_empty())?;
        let mut unique: HashSet<EntityId> = HashSet::new();
        validate_invariant(members.iter().all(|member| unique.insert(member.clone())))?;
        Ok(Self { id, name, members })
    }

    pub fn name(&self) -> &TeamName {
        &self.name
    }

    pub fn members(&self) -> &[EntityId] {
        &self.members
    }

    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    pub fn has_member(&self, member: &EntityId) -> bool {
        self.members.iter().any(|m| m == member)
    }

    pub fn add_member(&mut self, member: EntityId) -> Result<()> {
        validate_invariant(!self.has_member(&member))?;
        self.members.push(member);
        Ok(())
    }

    pub fn remove_member(&mut self, member: &EntityId) -> Result<()> {
        let original_len = self.members.len();
        self.members.retain(|m| m != member);
        validate_invariant(original_len != self.members.len())?;
        Ok(())
    }
}

impl AggregateRoot for Team {
    fn id(&self) -> &str {
        &self.id.0
    }
}

/// Value object describing a team's human-readable name.
#[derive(Debug, Clone)]
pub struct TeamName(String);

impl TeamName {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        validate_invariant(!value.trim().is_empty())?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> EntityId {
        EntityId(value.to_string())
    }

    #[test]
    fn team_requires_members() {
        let name = TeamName::new("Dream Team").unwrap();
        assert!(Team::new(id("team-1"), name, vec![]).is_err());
    }

    #[test]
    fn team_rejects_duplicate_members() {
        let name = TeamName::new("Dream Team").unwrap();
        let member = id("user-1");
        let result = Team::new(id("team-1"), name, vec![member.clone(), member]);
        assert!(result.is_err());
    }

    #[test]
    fn cannot_add_duplicate_member() {
        let name = TeamName::new("Dream Team").unwrap();
        let mut team = Team::new(id("team-1"), name, vec![id("user-1")]).unwrap();
        assert!(team.add_member(id("user-1")).is_err());
    }

    #[test]
    fn remove_member_requires_existing_member() {
        let name = TeamName::new("Dream Team").unwrap();
        let mut team = Team::new(id("team-1"), name, vec![id("user-1")]).unwrap();
        assert!(team.remove_member(&id("user-2")).is_err());
    }
}
