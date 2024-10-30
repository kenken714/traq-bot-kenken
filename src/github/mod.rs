pub(crate) mod client;
pub(crate) mod gql;

use std::{fmt, str::FromStr};

/// Represents personal access token.
#[derive(Clone)]
pub struct Pat {
    token: String,
}

impl FromStr for Pat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("ghp_") {
            return Err(anyhow::anyhow!("Invalid personal access token"));
        }
        Ok(Self {
            token: s.to_string(),
        })
    }
}
