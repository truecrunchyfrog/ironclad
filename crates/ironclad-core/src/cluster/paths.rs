use std::path::{Path, PathBuf};

use crate::{cell::id::CellId, cluster::cluster::Cluster};

impl Cluster {
    #[must_use]
    pub fn cluster_dir(path: &Path) -> PathBuf {
        path.join(".ironclad")
    }

    #[must_use]
    pub fn container_dir(&self) -> PathBuf {
        self.dir().parent().unwrap_or(self.dir()).to_path_buf()
    }

    #[must_use]
    pub fn cells_dir(&self) -> PathBuf {
        self.dir().join("cells")
    }

    #[must_use]
    pub fn cell_path(&self, id: &CellId) -> PathBuf {
        self.cells_dir().join(id.to_string())
    }

    #[must_use]
    pub fn snapshots_dir(&self) -> PathBuf {
        self.dir().join("snapshots")
    }

    #[must_use]
    pub fn snapshot_baseline_path(&self) -> PathBuf {
        self.snapshots_dir().join("baseline.json")
    }

    #[must_use]
    pub fn snapshot_pending_path(&self) -> PathBuf {
        self.snapshots_dir().join("pending.json")
    }
}
