//! Rating aggregates and supporting types.

use anyhow::{Context, Result};

/// Represents a user generated rating for a repository artifact.
#[derive(Debug, Clone, PartialEq)]
pub struct Rating {
    score: u8,
}

impl Rating {
    /// Construct a new rating ensuring the score is within 0..=100 boundaries.
    pub fn new(score: u8) -> Result<Self> {
        anyhow::ensure!(score <= 100, "score must be between 0 and 100");
        Ok(Self { score })
    }

    /// Return the normalized floating point value for downstream services.
    pub fn normalized(&self) -> f32 {
        f32::from(self.score) / 100.0
    }

    /// Merge another rating by averaging the scores.
    pub fn merge(&mut self, other: &Rating) {
        self.score = ((self.score as u16 + other.score as u16) / 2) as u8;
    }
}

/// Helper for mapping raw inputs into ratings.
pub fn parse_score(input: &str) -> Result<Rating> {
    let value = input
        .trim()
        .parse::<u8>()
        .with_context(|| format!("invalid rating score: {input}"))?;
    Rating::new(value)
}
