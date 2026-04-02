use std::{collections::HashMap, fs, path::Path};

use rayon::iter::{FromParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    cell::id::CellId,
    cluster::{Cluster, error::ClusterError},
    sample::batch::Batch,
    snapshot::{Snapshot, SnapshotEntry},
};

impl Cluster {
    pub fn capture_snapshot(&self, cache: Option<Snapshot>) -> Result<Snapshot, ClusterError> {
        let cells = self.load_cells()?;

        let batches = cells
            .par_iter()
            .map(|cell| {
                Ok((
                    cell.id().clone(),
                    cell.dependencies(),
                    match cache
                        .as_ref()
                        .and_then(|snapshot| snapshot.entries().get(cell.id()))
                    {
                        Some(entry)
                            if entry.batch().created().elapsed()? < *cell.cache_lifespan() =>
                        {
                            entry.batch().clone()
                        }
                        _ => cell.pipeline().eval(self)?,
                    },
                ))
            })
            .collect::<Result<Vec<_>, ClusterError>>()?;

        let cell_dependencies = |deps: &[CellId]| -> Result<HashMap<CellId, Batch>, ClusterError> {
            Ok(HashMap::from_iter(
                deps.iter()
                    .map(|dep_cell_id| -> Result<(CellId, Batch), ClusterError> {
                        Ok((
                            dep_cell_id.clone(),
                            batches
                                .iter()
                                .find_map(|(cell_id, _, batch)| {
                                    if cell_id == dep_cell_id {
                                        Some(batch.to_owned())
                                    } else {
                                        None
                                    }
                                })
                                .ok_or_else(|| {
                                    ClusterError::DependencyCellNotFound(dep_cell_id.clone())
                                })?,
                        ))
                    })
                    .collect::<Result<Vec<_>, ClusterError>>()?,
            ))
        };

        Ok(Snapshot::new(HashMap::from_par_iter(
            batches
                .par_iter()
                .map(|(cell_id, deps, batch)| {
                    Ok((
                        cell_id.clone(),
                        SnapshotEntry::new(batch.to_owned(), cell_dependencies(deps)?),
                    ))
                })
                .collect::<Result<Vec<_>, ClusterError>>()?,
        )))
    }

    fn load_snapshot(&self, path: &Path) -> Result<Snapshot, ClusterError> {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    fn save_snapshot(&self, path: &Path, snapshot: Snapshot) -> Result<(), ClusterError> {
        fs::write(path, serde_json::to_string_pretty(&snapshot)?)?;
        Ok(())
    }

    pub fn load_pending_snapshot(&self) -> Result<Snapshot, ClusterError> {
        self.load_snapshot(&self.snapshot_pending_path())
    }

    pub fn save_pending_snapshot(&self, snapshot: Snapshot) -> Result<(), ClusterError> {
        self.save_snapshot(&self.snapshot_pending_path(), snapshot)
    }

    pub fn load_baseline_snapshot(&self) -> Result<Snapshot, ClusterError> {
        self.load_snapshot(&self.snapshot_baseline_path())
    }

    pub fn save_baseline_snapshot(&self, snapshot: Snapshot) -> Result<(), ClusterError> {
        self.save_snapshot(&self.snapshot_baseline_path(), snapshot)
    }
}
