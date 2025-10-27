use anyhow::Result;

use crate::{
    model::{AggregateRoot, EntityId},
    validate_invariant,
};

/// Aggregate capturing the evaluation of a submission by a judge.
#[derive(Debug, Clone)]
pub struct Rating {
    id: EntityId,
    submission_id: EntityId,
    judge_id: EntityId,
    score: Score,
    feedback: Option<Feedback>,
}

impl Rating {
    pub fn new(
        id: EntityId,
        submission_id: EntityId,
        judge_id: EntityId,
        score: Score,
        feedback: Option<Feedback>,
    ) -> Result<Self> {
        if let Some(feedback) = &feedback {
            validate_invariant(!feedback.value().trim().is_empty())?;
        }
        Ok(Self {
            id,
            submission_id,
            judge_id,
            score,
            feedback,
        })
    }

    pub fn score(&self) -> &Score {
        &self.score
    }

    pub fn feedback(&self) -> Option<&Feedback> {
        self.feedback.as_ref()
    }
}

impl AggregateRoot for Rating {
    fn id(&self) -> &str {
        &self.id.0
    }
}

/// Value object representing the numeric score awarded to a submission.
#[derive(Debug, Clone)]
pub struct Score(u8);

impl Score {
    pub fn new(value: u8) -> Result<Self> {
        validate_invariant((1..=10).contains(&value))?;
        Ok(Self(value))
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

/// Optional qualitative feedback provided alongside the score.
#[derive(Debug, Clone)]
pub struct Feedback(String);

impl Feedback {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        validate_invariant(!value.trim().is_empty())?;
        validate_invariant(value.len() <= 1000)?;
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
    fn score_within_bounds() {
        assert!(Score::new(0).is_err());
        assert!(Score::new(11).is_err());
        assert!(Score::new(5).is_ok());
    }

    #[test]
    fn feedback_has_reasonable_length() {
        assert!(Feedback::new("a".repeat(1001)).is_err());
    }

    #[test]
    fn feedback_cannot_be_empty() {
        assert!(Feedback::new("   ").is_err());
    }
}
