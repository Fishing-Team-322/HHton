//! Search capabilities for indexing repositories and artifacts.

use anyhow::{Context, Result};

/// Represents a parsed search query issued by a user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchQuery {
    term: String,
}

impl SearchQuery {
    /// Parse free-form input into a sanitized search query.
    pub fn parse(input: &str) -> Result<Self> {
        let term = input.trim().to_lowercase();

        anyhow::ensure!(!term.is_empty(), "search term must not be empty");
        Ok(Self { term })
    }

    /// Convert the search query into tokens that can be fed into the index.
    pub fn tokens(&self) -> Result<Vec<String>> {
        let tokens = self
            .term
            .split_whitespace()
            .map(|token| token.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|token| !token.is_empty())
            .map(|token| token.to_string())
            .collect::<Vec<_>>();

        anyhow::ensure!(!tokens.is_empty(), "no searchable tokens produced");
        Ok(tokens)
    }
}

/// Converts a batch of raw inputs into validated search queries.
pub fn parse_many<'a, I>(inputs: I) -> Result<Vec<SearchQuery>>
where
    I: IntoIterator<Item = &'a str>,
{
    inputs
        .into_iter()
        .map(|input| {
            SearchQuery::parse(input).with_context(|| format!("invalid search query: {input}"))
        })
        .collect()
}
