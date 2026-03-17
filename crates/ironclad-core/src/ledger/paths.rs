use std::path::{Path, PathBuf};

use crate::{ledger::ledger::Ledger, node::id::NodeId};

impl Ledger {
    pub fn ledger_dir(path: &Path) -> PathBuf {
        path.join(".ironclad")
    }

    pub fn container_dir(&self) -> PathBuf {
        self.dir().parent().unwrap_or(self.dir()).to_path_buf()
    }

    pub fn nodes_dir(&self) -> PathBuf {
        self.dir().join("nodes")
    }

    pub fn node_path(&self, id: &NodeId) -> PathBuf {
        self.nodes_dir().join(&id.to_string())
    }

    pub fn snapshots_dir(&self) -> PathBuf {
        self.dir().join("snapshots")
    }

    pub fn snapshot_baseline_path(&self) -> PathBuf {
        self.snapshots_dir().join("baseline.json")
    }

    pub fn snapshot_pending_path(&self) -> PathBuf {
        self.snapshots_dir().join("pending.json")
    }
}
