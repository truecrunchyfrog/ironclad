use std::{fs, path::Path};

use log::info;

use crate::catalog::{catalog::Catalog, error::CatalogError};

const GITIGNORE_CONTENT: &str = "snapshots/candidate.json";

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
        fs::create_dir(catalog.dir())?;

        let gitignore_path = catalog.dir().join(".gitignore");
        info!("creating {gitignore_path:#?}");
        fs::write(gitignore_path, GITIGNORE_CONTENT)?;

        {
            let facts_dir = catalog.facts_dir_path();
            info!("creating {facts_dir:#?}");
            fs::create_dir(facts_dir)?;
        }

        {
            let snapshots_dir = catalog.snapshots_dir_path();
            info!("creating {snapshots_dir:#?}");
            fs::create_dir(snapshots_dir)?;

            {
                let snapshot_baseline_path = catalog.snapshot_baseline_file_path();
                info!("creating {snapshot_baseline_path:#?}");
                fs::write(snapshot_baseline_path, "{}")?;
            }

            {
                let snapshot_candidate_path = catalog.snapshot_candidate_file_path();
                info!("creating {snapshot_candidate_path:#?}");
                fs::write(snapshot_candidate_path, "{}")?;
            }
        }
    }

    Ok(())
}
