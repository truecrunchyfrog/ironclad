use ironclad_core::{catalog::Catalog, fact::id::FactId, sample::batch::Batch, snapshot::Snapshot};

pub(crate) fn resolve_catalog() -> anyhow::Result<Catalog> {
    Ok(Catalog::find_for_working_dir(&std::env::current_dir()?)?)
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
