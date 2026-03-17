use std::path::{Path, PathBuf};

pub struct Ledger {
    dir: PathBuf,
}

impl Ledger {
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }
}
