use std::path::Path;

use log::info;

use crate::{
    catalog::{catalog::Catalog, error::CatalogError, fact_index::FactIndex},
    snapshot::Snapshot,
};

impl Catalog {
    pub fn create_catalog(working_dir: &Path) -> Result<Catalog, CatalogError> {
        let catalog = Catalog::new(Catalog::catalog_dir_path(working_dir));

        populate_catalog_dir(&catalog)?;

        Ok(catalog)
    }
}

fn populate_catalog_dir(catalog: &Catalog) -> Result<(), CatalogError> {
    if catalog.dir().try_exists()? {
        if catalog.dir().is_dir() {
            return Err(CatalogError::PathAlreadyExists(catalog.dir().to_path_buf()));
        }

        return Err(CatalogError::PathNotDirectory(catalog.dir().to_path_buf()));
    }

    {
        info!("creating {:#?}", catalog.dir());
        std::fs::create_dir(catalog.dir())?;

        let gitignore_file_path = catalog.dir().join(".gitignore");
        info!("creating {gitignore_file_path:#?}");
        std::fs::write(
            gitignore_file_path,
            catalog
                .snapshot_actual_file_path()
                .strip_prefix(catalog.dir())?
                .to_str()
                .unwrap(),
        )?;

        let index_file_path = catalog.fact_index_file_path();
        info!("creating {index_file_path:#?}");
        std::fs::write(index_file_path, toml::to_string_pretty(&FactIndex::new())?)?;

        let facts_dir = catalog.facts_dir_path();
        info!("creating {facts_dir:#?}");
        std::fs::create_dir(facts_dir)?;

        let snapshots_dir = catalog.snapshots_dir_path();
        info!("creating {snapshots_dir:#?}");
        std::fs::create_dir(snapshots_dir)?;

        let snapshot_canon_file_path = catalog.snapshot_canon_file_path();
        info!("creating {snapshot_canon_file_path:#?}");
        std::fs::write(
            snapshot_canon_file_path,
            serde_json::to_string_pretty(&Snapshot::default())?,
        )?;
    }

    Ok(())
}
