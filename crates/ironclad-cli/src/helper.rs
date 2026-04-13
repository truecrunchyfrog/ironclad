use std::collections::HashMap;

use ironclad_core::{
    catalog::Catalog,
    fact::id::FactId,
    sample::batch::Batch,
    snapshot::{Snapshot, diff::BatchDiff},
};

pub(crate) fn resolve_catalog() -> anyhow::Result<Catalog> {
    Ok(Catalog::find_for_working_dir(&std::env::current_dir()?)?)
}

// TODO ???
pub(crate) fn collect_changed_snapshot_diffs(
    snapshot_diff: HashMap<FactId, BatchDiff>,
) -> Vec<(FactId, BatchDiff)> {
    snapshot_diff
        .into_iter()
        .filter(|(_, diff)| !diff.batches_equal())
        .collect::<Vec<_>>()
}

pub(crate) fn set_snapshot_batch<'a>(
    fact_id: &'a FactId,
    snapshot: &'a mut Snapshot,
    batch: Option<Batch>,
) {
    match batch {
        Some(batch) => {
            *snapshot.entries_mut().entry(fact_id.clone()).or_default() = batch;
        }
        None => {
            snapshot.entries_mut().remove(fact_id);
        }
    }
}
