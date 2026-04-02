use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Trace(HashMap<String, String>);

impl Trace {
    #[must_use]
    pub fn new(values: HashMap<String, String>) -> Self {
        Self(values)
    }
}
