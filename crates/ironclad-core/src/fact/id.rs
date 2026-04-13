use std::{fmt::Display, path::Path, str::FromStr};

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
}

impl Default for FactId {
    fn default() -> Self {
        Self(String::from("[no id assigned]"))
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
