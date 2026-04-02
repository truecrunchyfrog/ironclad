use std::path::{Path, PathBuf};

use crate::{cell::id::CellId, ledger::ledger::Ledger};

impl Ledger {
    pub fn ledger_dir(path: &Path) -> PathBuf {
        path.join(".ironclad")
    }

    pub fn container_dir(&self) -> PathBuf {
        self.dir().parent().unwrap_or(self.dir()).to_path_buf()
    }

    pub fn cells_dir(&self) -> PathBuf {
        self.dir().join("cells")
    }

    pub fn cell_path(&self, id: &CellId) -> PathBuf {
        self.cells_dir().join(id.to_string())
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
