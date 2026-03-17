use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{node::id::NodeId, sample::batch::Batch};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SnapshotEntry {
    batch: Batch,
    dependencies: HashMap<NodeId, Batch>,
}

impl SnapshotEntry {
    pub fn new(batch: Batch, dependencies: HashMap<NodeId, Batch>) -> Self {
        Self {
            batch,
            dependencies,
        }
    }

    pub fn batch(&self) -> &Batch {
        &self.batch
    }

    pub fn batch_mut(&mut self) -> &mut Batch {
        &mut self.batch
    }

    pub fn dependencies(&self) -> &HashMap<NodeId, Batch> {
        &self.dependencies
    }

    pub fn dependencies_mut(&mut self) -> &mut HashMap<NodeId, Batch> {
        &mut self.dependencies
    }
}
