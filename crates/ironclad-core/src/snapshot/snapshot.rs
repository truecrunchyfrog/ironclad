use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{cell::id::CellId, snapshot::entry::SnapshotEntry};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Snapshot(HashMap<CellId, SnapshotEntry>);

impl Snapshot {
    #[must_use] 
    pub fn new(entries: HashMap<CellId, SnapshotEntry>) -> Self {
        Self(entries)
    }

    #[must_use] 
    pub fn entries(&self) -> &HashMap<CellId, SnapshotEntry> {
        &self.0
    }

    pub fn entries_mut(&mut self) -> &mut HashMap<CellId, SnapshotEntry> {
        &mut self.0
    }
}
