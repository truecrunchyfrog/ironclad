use std::{collections::HashMap, fs, path::Path};

use rayon::iter::{FromParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    ledger::{Ledger, error::LedgerError},
    node::id::NodeId,
    sample::batch::Batch,
    snapshot::{Snapshot, SnapshotEntry},
};

impl Ledger {
    pub fn capture_snapshot(&self, cache: Option<Snapshot>) -> Result<Snapshot, LedgerError> {
        let nodes = self.load_nodes()?;

        let batches = nodes
            .par_iter()
            .map(|node| {
                Ok((
                    node.id().clone(),
                    node.dependencies(),
                    match cache
                        .as_ref()
                        .map(|snapshot| snapshot.entries().get(node.id()))
                        .flatten()
                    {
                        Some(entry)
                            if entry.batch().created().elapsed()? < *node.cache_lifespan() =>
                        {
                            entry.batch().clone()
                        }
                        _ => node.pipeline().eval(self)?,
                    },
                ))
            })
            .collect::<Result<Vec<_>, LedgerError>>()?;

        let node_dependencies = |deps: &[NodeId]| -> Result<HashMap<NodeId, Batch>, LedgerError> {
            Ok(HashMap::from_iter(
                deps.iter()
                    .map(|dep_node_id| -> Result<(NodeId, Batch), LedgerError> {
                        Ok((
                            dep_node_id.clone(),
                            batches
                                .iter()
                                .find_map(|(node_id, _, batch)| {
                                    if node_id == dep_node_id {
                                        Some(batch.to_owned())
                                    } else {
                                        None
                                    }
                                })
                                .ok_or_else(|| {
                                    LedgerError::DependencyNodeNotFound(dep_node_id.clone())
                                })?,
                        ))
                    })
                    .collect::<Result<Vec<_>, LedgerError>>()?,
            ))
        };

        Ok(Snapshot::new(HashMap::from_par_iter(
            batches
                .par_iter()
                .map(|(node_id, deps, batch)| {
                    Ok((
                        node_id.clone(),
                        SnapshotEntry::new(batch.to_owned(), node_dependencies(deps)?),
                    ))
                })
                .collect::<Result<Vec<_>, LedgerError>>()?,
        )))
    }

    fn load_snapshot(&self, path: &Path) -> Result<Snapshot, LedgerError> {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    fn save_snapshot(&self, path: &Path, snapshot: Snapshot) -> Result<(), LedgerError> {
        fs::write(path, serde_json::to_string_pretty(&snapshot)?)?;
        Ok(())
    }

    pub fn load_pending_snapshot(&self) -> Result<Snapshot, LedgerError> {
        self.load_snapshot(&self.snapshot_pending_path())
    }

    pub fn save_pending_snapshot(&self, snapshot: Snapshot) -> Result<(), LedgerError> {
        self.save_snapshot(&self.snapshot_pending_path(), snapshot)
    }

    pub fn load_baseline_snapshot(&self) -> Result<Snapshot, LedgerError> {
        self.load_snapshot(&self.snapshot_baseline_path())
    }

    pub fn save_baseline_snapshot(&self, snapshot: Snapshot) -> Result<(), LedgerError> {
        self.save_snapshot(&self.snapshot_baseline_path(), snapshot)
    }
}
