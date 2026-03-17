use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{node::id::NodeId, snapshot::entry::SnapshotEntry};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Snapshot(HashMap<NodeId, SnapshotEntry>);

impl Snapshot {
    pub fn new(entries: HashMap<NodeId, SnapshotEntry>) -> Self {
        Self(entries)
    }

    pub fn entries(&self) -> &HashMap<NodeId, SnapshotEntry> {
        &self.0
    }

    pub fn entries_mut(&mut self) -> &mut HashMap<NodeId, SnapshotEntry> {
        &mut self.0
    }
}
