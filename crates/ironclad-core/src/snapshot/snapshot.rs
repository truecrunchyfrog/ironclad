use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{fact::id::FactId, sample::batch::Batch};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Snapshot(HashMap<FactId, Batch>);

impl Snapshot {
    #[must_use]
    pub fn new(entries: HashMap<FactId, Batch>) -> Self {
        Self(entries)
    }

    #[must_use]
    pub fn entries(&self) -> &HashMap<FactId, Batch> {
        &self.0
    }

    pub fn entries_mut(&mut self) -> &mut HashMap<FactId, Batch> {
        &mut self.0
    }
}
