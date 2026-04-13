use std::path::{Path, PathBuf};

use crate::{fact::id::FactId, catalog::catalog::Catalog};

impl Catalog {
    #[must_use]
    pub fn catalog_dir(path: &Path) -> PathBuf {
        path.join(".ironclad")
    }

    #[must_use]
    pub fn container_dir(&self) -> PathBuf {
        self.dir().parent().unwrap_or(self.dir()).to_path_buf()
    }

    #[must_use]
    pub fn facts_dir(&self) -> PathBuf {
        self.dir().join("facts")
    }

    #[must_use]
    pub fn fact_path(&self, id: &FactId) -> PathBuf {
        self.facts_dir().join(id.to_string())
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
