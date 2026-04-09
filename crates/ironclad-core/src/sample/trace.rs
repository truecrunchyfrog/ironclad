use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Trace(HashMap<String, String>);

impl Trace {
    #[must_use]
    pub fn new(values: HashMap<String, String>) -> Self {
        Self(values)
    }
}

impl Hash for Trace {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut trace_items = self.0.iter().collect::<Vec<_>>();
        trace_items.sort();
        trace_items.hash(state);
    }
}
