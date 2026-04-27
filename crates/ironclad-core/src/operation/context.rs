use std::path::{Path, PathBuf};

use crate::catalog::Catalog;

#[derive(Clone)]
pub struct OperationContext {
    working_dir: PathBuf,
    catalog: Option<Catalog>,
}

impl OperationContext {
    #[must_use]
    pub fn for_working_dir(working_dir: PathBuf) -> Self {
        Self {
            working_dir,
            catalog: None,
        }
    }

    #[must_use]
    pub fn with_catalog(catalog: Catalog) -> Self {
        Self {
            working_dir: catalog.container_dir_path(),
            catalog: Some(catalog),
        }
    }

    #[must_use]
    pub fn working_dir(&self) -> &Path {
        &self.working_dir
    }

    #[must_use]
    pub fn catalog(&self) -> Option<&Catalog> {
        self.catalog.as_ref()
    }
}
