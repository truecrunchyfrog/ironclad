use std::{fmt::Display, path::Path, str::FromStr};

use rand::{RngCore, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub struct FactId(String);

impl FactId {
    #[must_use]
    pub fn for_path(path: &Path) -> Self {
        Self(
            path.file_name()
                .expect("fact path should have file name")
                .to_str()
                .expect("fact file name should be UTF-8")
                .to_string(),
        )
    }

    pub fn random(rng: Option<ThreadRng>) -> Self {
        let mut bytes = [0u8; 4];
        rng.unwrap_or_else(rand::rng).fill_bytes(&mut bytes);
        Self(hex::encode(bytes))
    }
}

impl Default for FactId {
    fn default() -> Self {
        Self::random(None)
    }
}

impl PartialEq for FactId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for FactId {}

impl Display for FactId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for FactId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl FromStr for FactId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
