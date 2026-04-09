use std::collections::HashMap;

use anyhow::anyhow;
use ironclad_core::{
    cell::{Cell, id::CellId},
    cluster::Cluster,
    sample::batch::Batch,
    snapshot::{Snapshot, diff::BatchDiff},
};

use crate::{batch_origin::BatchOrigin, reuse_cell};

pub(crate) fn resolve_cluster() -> anyhow::Result<Cluster> {
    Ok(Cluster::find_for_working_dir(&std::env::current_dir()?)?)
}

fn explicit_or_reused_cell_id(
    cluster: &Cluster,
    specified_cell_id: Option<String>,
) -> anyhow::Result<String> {
    match specified_cell_id {
        Some(cell_id) if cell_id != "-" => Ok(cell_id),
        _ => reuse_cell::get(cluster)?
            .map(|cell_id| cell_id.to_string())
            .ok_or(anyhow!("cell ID not specified, and not reusing")),
    }
}

pub(crate) fn resolve_explicit_or_reused_cell_id(
    cluster: &Cluster,
    specified_cell_id: Option<String>,
) -> anyhow::Result<CellId> {
    Ok(cluster.resolve_cell_id(&explicit_or_reused_cell_id(cluster, specified_cell_id)?)?)
}

pub(crate) fn resolve_explicit_or_reused_cell(
    cluster: &Cluster,
    specified_cell_id: Option<String>,
) -> anyhow::Result<Cell> {
    Ok(cluster.resolve_cell(&explicit_or_reused_cell_id(cluster, specified_cell_id)?)?)
}

pub(crate) fn collect_changed_snapshot_diffs(
    snapshot_diff: HashMap<CellId, (BatchDiff, Vec<(CellId, BatchDiff)>)>,
) -> Vec<(BatchOrigin, BatchDiff)> {
    snapshot_diff
        .into_iter()
        .flat_map(|(cell_id, (diff, dep_diffs))| {
            std::iter::once((BatchOrigin::DirtyCell(cell_id.clone()), diff))
                .chain(dep_diffs.into_iter().map(|(dep_cell_id, dep_diff)| {
                    (
                        BatchOrigin::StaleDependencyCell {
                            dependency: dep_cell_id,
                            dependent: cell_id.clone(),
                        },
                        dep_diff,
                    )
                }))
                .collect::<Vec<_>>()
        })
        .filter(|(_, diff)| !diff.batches_equal())
        .collect::<Vec<_>>()
}

pub(crate) fn find_batch_diff<'a>(
    origin: &'a BatchOrigin,
    diffs: &'a HashMap<CellId, (BatchDiff, Vec<(CellId, BatchDiff)>)>,
) -> Option<&'a BatchDiff> {
    match origin {
        BatchOrigin::DirtyCell(cell_id) => diffs.get(cell_id).map(|(diff, _)| diff),
        BatchOrigin::StaleDependencyCell {
            dependency,
            dependent,
        } => diffs
            .get(dependent)
            .map(|(_, dep_diffs)| {
                dep_diffs.iter().find_map(|(dep_cell_id, diff)| {
                    if dep_cell_id == dependency {
                        Some(diff)
                    } else {
                        None
                    }
                })
            })
            .flatten(),
    }
}

pub(crate) fn find_batch_diff_mut<'a>(
    origin: &'a BatchOrigin,
    diffs: &'a mut HashMap<CellId, (BatchDiff, Vec<(CellId, BatchDiff)>)>,
) -> Option<&'a mut BatchDiff> {
    match origin {
        BatchOrigin::DirtyCell(cell_id) => diffs.get_mut(cell_id).map(|(diff, _)| diff),
        BatchOrigin::StaleDependencyCell {
            dependency,
            dependent,
        } => diffs
            .get_mut(dependent)
            .map(|(_, dep_diffs)| {
                dep_diffs.iter_mut().find_map(|(dep_cell_id, diff)| {
                    if dep_cell_id == dependency {
                        Some(diff)
                    } else {
                        None
                    }
                })
            })
            .flatten(),
    }
}

pub(crate) fn set_snapshot_batch<'a>(
    origin: &'a BatchOrigin,
    snapshot: &'a mut Snapshot,
    batch: Option<Batch>,
) {
    match origin {
        BatchOrigin::DirtyCell(cell_id) => match batch {
            Some(batch) => {
                *snapshot
                    .entries_mut()
                    .entry(cell_id.clone())
                    .or_default()
                    .batch_mut() = batch;
            }
            None => {
                snapshot.entries_mut().remove(cell_id);
            }
        },
        BatchOrigin::StaleDependencyCell {
            dependency,
            dependent,
        } => match batch {
            Some(batch) => {
                *snapshot
                    .entries_mut()
                    .entry(dependent.clone())
                    .or_default()
                    .dependencies_mut()
                    .entry(dependency.clone())
                    .or_default() = batch;
            }
            None => {
                snapshot
                    .entries_mut()
                    .entry(dependent.clone())
                    .or_default()
                    .dependencies_mut()
                    .remove(dependency);
            }
        },
    };
}
