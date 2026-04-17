use std::path::{Path, PathBuf};

use crate::catalog::catalog::Catalog;

impl Catalog {
    #[must_use]
    pub fn catalog_dir_path(path: &Path) -> PathBuf {
        path.join(".ironclad")
    }

    #[must_use]
    pub fn container_dir_path(&self) -> PathBuf {
        self.dir().parent().unwrap_or(self.dir()).to_path_buf()
    }

    #[must_use]
    pub fn facts_dir_path(&self) -> PathBuf {
        self.dir().join("facts")
    }

    #[must_use]
    pub fn fact_index_file_path(&self) -> PathBuf {
        self.dir().join("index.json")
    }

    #[must_use]
    pub fn fact_file_path(&self, fact_id: &str) -> PathBuf {
        self.facts_dir_path().join(fact_id)
    }

    #[must_use]
    pub fn snapshots_dir_path(&self) -> PathBuf {
        self.dir().join("snapshots")
    }

    #[must_use]
    pub fn snapshot_baseline_file_path(&self) -> PathBuf {
        self.snapshots_dir_path().join("baseline.json")
    }

    #[must_use]
    pub fn snapshot_candidate_file_path(&self) -> PathBuf {
        self.snapshots_dir_path().join("candidate.json")
    }
}
