use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{fact::id::FactId, snapshot::entry::SnapshotEntry};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Snapshot(HashMap<FactId, SnapshotEntry>);

impl Snapshot {
    #[must_use]
    pub fn new(entries: HashMap<FactId, SnapshotEntry>) -> Self {
        Self(entries)
    }

    #[must_use]
    pub fn entries(&self) -> &HashMap<FactId, SnapshotEntry> {
        &self.0
    }

    pub fn entries_mut(&mut self) -> &mut HashMap<FactId, SnapshotEntry> {
        &mut self.0
    }
}
