use std::{fmt::Display, path::Path, str::FromStr};

use rand::{RngCore, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub struct NodeId(String);

impl NodeId {
    pub fn for_path(path: &Path) -> Self {
        Self(
            path.file_name()
                .expect("node path should have file name")
                .to_str()
                .expect("node file name should be UTF-8")
                .to_string(),
        )
    }

    pub fn random(rng: Option<ThreadRng>) -> Self {
        let mut bytes = [0u8; 4];
        rng.unwrap_or_else(rand::rng).fill_bytes(&mut bytes);
        Self(hex::encode(bytes))
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::random(None)
    }
}

impl PartialEq for NodeId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for NodeId {}

impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for NodeId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl FromStr for NodeId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
