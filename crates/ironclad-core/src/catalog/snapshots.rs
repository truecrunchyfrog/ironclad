use std::{collections::HashMap, fs, path::Path};

use rayon::iter::{FromParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    catalog::{Catalog, error::CatalogError},
    snapshot::Snapshot,
};

impl Catalog {
    pub fn capture_snapshot(&self, cache: Option<Snapshot>) -> Result<Snapshot, CatalogError> {
        let facts = self.load_facts()?;

        let batches = facts
            .par_iter()
            .map(|fact| {
                Ok((
                    fact.id().clone(),
                    match cache
                        .as_ref()
                        .and_then(|snapshot| snapshot.entries().get(fact.id()))
                    {
                        Some(entry) if entry.created().elapsed()? < *fact.cache_lifespan() => {
                            entry.clone()
                        }
                        _ => fact.recipe().eval(self)?,
                    },
                ))
            })
            .collect::<Result<Vec<_>, CatalogError>>()?;

        Ok(Snapshot::new(HashMap::from_par_iter(
            batches
                // TODO to par or not to par?
                .par_iter()
                .map(|(fact_id, batch)| Ok((fact_id.clone(), batch.to_owned())))
                .collect::<Result<Vec<_>, CatalogError>>()?,
        )))
    }

    fn load_snapshot(&self, path: &Path) -> Result<Snapshot, CatalogError> {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    fn save_snapshot(&self, path: &Path, snapshot: Snapshot) -> Result<(), CatalogError> {
        fs::write(path, serde_json::to_string_pretty(&snapshot)?)?;
        Ok(())
    }

    pub fn load_candidate_snapshot(&self) -> Result<Snapshot, CatalogError> {
        self.load_snapshot(&self.snapshot_candidate_path())
    }

    pub fn save_candidate_snapshot(&self, snapshot: Snapshot) -> Result<(), CatalogError> {
        self.save_snapshot(&self.snapshot_candidate_path(), snapshot)
    }

    pub fn load_baseline_snapshot(&self) -> Result<Snapshot, CatalogError> {
        self.load_snapshot(&self.snapshot_baseline_path())
    }

    pub fn save_baseline_snapshot(&self, snapshot: Snapshot) -> Result<(), CatalogError> {
        self.save_snapshot(&self.snapshot_baseline_path(), snapshot)
    }
}
