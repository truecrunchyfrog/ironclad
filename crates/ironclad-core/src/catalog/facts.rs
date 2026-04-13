use std::{
    fs::{self, DirEntry},
    path::Path,
};

use log::{info, warn};

use crate::{
    catalog::{catalog::Catalog, error::CatalogError},
    fact::{Fact, error::FactError, id::FactId},
};

impl Catalog {
    fn fact_files(&self) -> Vec<DirEntry> {
        let facts_dir = self.facts_dir();
        let entries = facts_dir
            .read_dir()
            .unwrap_or_else(|_| panic!("cannot read {facts_dir:#?} as directory"));

        entries
            .filter_map(|entry| {
                entry
                    .inspect_err(|err| warn!("strange entry: {err}"))
                    .ok()
                    .filter(|entry| {
                        entry
                            .file_type()
                            .inspect_err(|err| warn!("cannot check file type: {err}"))
                            .is_ok_and(|filetype| {
                                if filetype.is_file() {
                                    true
                                } else {
                                    warn!("non-file fact entry ignored: {:#?}", entry.path());
                                    false
                                }
                            })
                    })
            })
            .collect::<Vec<_>>()
    }

    #[must_use]
    pub fn fact_ids(&self) -> Vec<FactId> {
        self.fact_files()
            .iter()
            .map(|file| FactId::for_path(&file.path()))
            .collect()
    }

    pub fn load_facts(&self) -> Result<Vec<Fact>, CatalogError> {
        Ok(self
            .fact_files()
            .iter()
            .flat_map(|entry| {
                self.load_fact_for_path(&entry.path())
                    .inspect_err(|err| warn!("failed to load fact {:#?}: {}", entry.path(), err))
            })
            .collect::<Vec<_>>())
    }

    pub fn load_fact_for_path(&self, path: &Path) -> Result<Fact, FactError> {
        if !path.try_exists()? {
            return Err(FactError::PathNotFound(path.to_path_buf()));
        }

        let mut fact: Fact = serde_json::from_str(&fs::read_to_string(path)?)?;
        *fact.id_mut() = FactId::for_path(path);

        Ok(fact)
    }

    pub fn load_fact_for_id(&self, id: &FactId) -> Result<Fact, FactError> {
        self.load_fact_for_path(&self.fact_path(id))
    }

    pub fn save_fact(&self, fact: &Fact) -> Result<(), FactError> {
        let path = self.fact_path(fact.id());

        if !path.try_exists()? {
            return Err(FactError::PathNotFound(path));
        }

        write_fact(&path, fact)?;

        Ok(())
    }

    pub fn add_fact(&self, fact: &Fact) -> Result<(), FactError> {
        let path = self.fact_path(fact.id());

        if path.try_exists()? {
            return Err(FactError::PathAlreadyExists(path));
        }

        write_fact(&path, fact)?;

        Ok(())
    }

    pub fn remove_fact(&self, id: &FactId) -> Result<(), CatalogError> {
        let path = self.fact_path(id);

        if !path.try_exists()? {
            return Err(FactError::PathNotFound(path).into());
        }

        info!("removing fact at {path:?}");
        fs::remove_file(path)?;

        Ok(())
    }
}

fn write_fact(path: &Path, fact: &Fact) -> Result<(), FactError> {
    info!("writing fact at {path:?}");
    fs::write(path, serde_json::to_vec_pretty(fact)?)?;
    Ok(())
}
