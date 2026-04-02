use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{cell::id::CellId, sample::batch::Batch};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SnapshotEntry {
    batch: Batch,
    dependencies: HashMap<CellId, Batch>,
}

impl SnapshotEntry {
    #[must_use]
    pub fn new(batch: Batch, dependencies: HashMap<CellId, Batch>) -> Self {
        Self {
            batch,
            dependencies,
        }
    }

    #[must_use]
    pub fn batch(&self) -> &Batch {
        &self.batch
    }

    pub fn batch_mut(&mut self) -> &mut Batch {
        &mut self.batch
    }

    #[must_use]
    pub fn dependencies(&self) -> &HashMap<CellId, Batch> {
        &self.dependencies
    }

    pub fn dependencies_mut(&mut self) -> &mut HashMap<CellId, Batch> {
        &mut self.dependencies
    }
}
