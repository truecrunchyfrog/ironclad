use std::{collections::HashMap, fs, path::Path};

use rayon::iter::{FromParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    fact::id::FactId,
    catalog::{Catalog, error::CatalogError},
    sample::batch::Batch,
    snapshot::{Snapshot, SnapshotEntry},
};

impl Catalog {
    pub fn capture_snapshot(&self, cache: Option<Snapshot>) -> Result<Snapshot, CatalogError> {
        let facts = self.load_facts()?;

        let batches = facts
            .par_iter()
            .map(|fact| {
                Ok((
                    fact.id().clone(),
                    fact.dependencies(),
                    match cache
                        .as_ref()
                        .and_then(|snapshot| snapshot.entries().get(fact.id()))
                    {
                        Some(entry)
                            if entry.batch().created().elapsed()? < *fact.cache_lifespan() =>
                        {
                            entry.batch().clone()
                        }
                        _ => fact.schema().eval(self)?,
                    },
                ))
            })
            .collect::<Result<Vec<_>, CatalogError>>()?;

        let fact_dependencies = |deps: &[FactId]| -> Result<HashMap<FactId, Batch>, CatalogError> {
            Ok(HashMap::from_iter(
                deps.iter()
                    .map(|dep_fact_id| -> Result<(FactId, Batch), CatalogError> {
                        Ok((
                            dep_fact_id.clone(),
                            batches
                                .iter()
                                .find_map(|(fact_id, _, batch)| {
                                    if fact_id == dep_fact_id {
                                        Some(batch.to_owned())
                                    } else {
                                        None
                                    }
                                })
                                .ok_or_else(|| {
                                    CatalogError::DependencyFactNotFound(dep_fact_id.clone())
                                })?,
                        ))
                    })
                    .collect::<Result<Vec<_>, CatalogError>>()?,
            ))
        };

        Ok(Snapshot::new(HashMap::from_par_iter(
            batches
                .par_iter()
                .map(|(fact_id, deps, batch)| {
                    Ok((
                        fact_id.clone(),
                        SnapshotEntry::new(batch.to_owned(), fact_dependencies(deps)?),
                    ))
                })
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

    pub fn load_pending_snapshot(&self) -> Result<Snapshot, CatalogError> {
        self.load_snapshot(&self.snapshot_pending_path())
    }

    pub fn save_pending_snapshot(&self, snapshot: Snapshot) -> Result<(), CatalogError> {
        self.save_snapshot(&self.snapshot_pending_path(), snapshot)
    }

    pub fn load_baseline_snapshot(&self) -> Result<Snapshot, CatalogError> {
        self.load_snapshot(&self.snapshot_baseline_path())
    }

    pub fn save_baseline_snapshot(&self, snapshot: Snapshot) -> Result<(), CatalogError> {
        self.save_snapshot(&self.snapshot_baseline_path(), snapshot)
    }
}
