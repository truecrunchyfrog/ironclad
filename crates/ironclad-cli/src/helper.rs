use std::collections::HashMap;

use ironclad_core::{
    catalog::Catalog,
    fact::id::FactId,
    sample::batch::Batch,
    snapshot::{Snapshot, diff::BatchDiff},
};

use crate::batch_origin::BatchOrigin;

pub(crate) fn resolve_catalog() -> anyhow::Result<Catalog> {
    Ok(Catalog::find_for_working_dir(&std::env::current_dir()?)?)
}

pub(crate) fn collect_changed_snapshot_diffs(
    snapshot_diff: HashMap<FactId, (BatchDiff, Vec<(FactId, BatchDiff)>)>,
) -> Vec<(BatchOrigin, BatchDiff)> {
    snapshot_diff
        .into_iter()
        .flat_map(|(fact_id, (diff, dep_diffs))| {
            std::iter::once((BatchOrigin::DirtyFact(fact_id.clone()), diff))
                .chain(dep_diffs.into_iter().map(|(dep_fact_id, dep_diff)| {
                    (
                        BatchOrigin::StaleDependencyFact {
                            dependency: dep_fact_id,
                            dependent: fact_id.clone(),
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
    diffs: &'a HashMap<FactId, (BatchDiff, Vec<(FactId, BatchDiff)>)>,
) -> Option<&'a BatchDiff> {
    match origin {
        BatchOrigin::DirtyFact(fact_id) => diffs.get(fact_id).map(|(diff, _)| diff),
        BatchOrigin::StaleDependencyFact {
            dependency,
            dependent,
        } => diffs
            .get(dependent)
            .map(|(_, dep_diffs)| {
                dep_diffs.iter().find_map(|(dep_fact_id, diff)| {
                    if dep_fact_id == dependency {
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
    diffs: &'a mut HashMap<FactId, (BatchDiff, Vec<(FactId, BatchDiff)>)>,
) -> Option<&'a mut BatchDiff> {
    match origin {
        BatchOrigin::DirtyFact(fact_id) => diffs.get_mut(fact_id).map(|(diff, _)| diff),
        BatchOrigin::StaleDependencyFact {
            dependency,
            dependent,
        } => diffs
            .get_mut(dependent)
            .map(|(_, dep_diffs)| {
                dep_diffs.iter_mut().find_map(|(dep_fact_id, diff)| {
                    if dep_fact_id == dependency {
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
        BatchOrigin::DirtyFact(fact_id) => match batch {
            Some(batch) => {
                *snapshot
                    .entries_mut()
                    .entry(fact_id.clone())
                    .or_default()
                    .batch_mut() = batch;
            }
            None => {
                snapshot.entries_mut().remove(fact_id);
            }
        },
        BatchOrigin::StaleDependencyFact {
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
