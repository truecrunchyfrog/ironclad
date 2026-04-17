use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::sample::batch::Batch;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Snapshot(HashMap<String, Batch>);

impl Snapshot {
    #[must_use]
    pub fn new(entries: HashMap<String, Batch>) -> Self {
        Self(entries)
    }

    #[must_use]
    pub fn into_entries(self) -> HashMap<String, Batch> {
        self.0
    }

    #[must_use]
    pub fn entries(&self) -> &HashMap<String, Batch> {
        &self.0
    }

    pub fn entries_mut(&mut self) -> &mut HashMap<String, Batch> {
        &mut self.0
    }
}
