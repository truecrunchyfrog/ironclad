use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct Catalog {
    dir: PathBuf,
}

impl Catalog {
    #[must_use]
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    #[must_use]
    pub fn dir(&self) -> &Path {
        &self.dir
    }
}
