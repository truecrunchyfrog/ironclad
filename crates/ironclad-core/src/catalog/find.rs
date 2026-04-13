use std::{ffi::OsStr, path::Path};

use crate::catalog::{Catalog, error::CatalogError};

impl Catalog {
    pub(crate) fn is_catalog_dir(path: &Path) -> bool {
        path.file_name() == Some(OsStr::new(".ironclad")) && path.is_dir()
    }

    pub fn find_for_working_dir(working_dir: &Path) -> Result<Catalog, CatalogError> {
        working_dir
            .ancestors()
            .find_map(|ancestor| {
                ancestor.read_dir().ok().and_then(|read_dir| {
                    read_dir
                        .flatten()
                        .map(|child| child.path())
                        .find(|child| Catalog::is_catalog_dir(child))
                        .map(Catalog::new)
                })
            })
            .ok_or_else(|| CatalogError::PathNotFound(working_dir.to_path_buf()))
    }
}
