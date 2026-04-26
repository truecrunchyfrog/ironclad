use std::path::Path;

use crate::catalog::{Catalog, error::CatalogError};

impl Catalog {
    pub fn find_for_working_dir(working_dir: &Path) -> Result<Catalog, CatalogError> {
        for ancestor in working_dir.ancestors() {
            let path = Catalog::catalog_dir_path(ancestor);

            if path.try_exists()? {
                if path.is_dir() {
                    return Ok(Catalog::new(path));
                }

                return Err(CatalogError::PathNotDirectory(path));
            }
        }

        Err(CatalogError::PathNotFound(working_dir.to_path_buf()))
    }
}
