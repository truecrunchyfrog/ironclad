use log::info;

use crate::{
    args::dependency::remove::RemoveDependencyArgs,
    helper::{resolve_explicit_or_reused_cell_id, resolve_ledger},
};

pub(super) fn dispatch(args: RemoveDependencyArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;

    let dependents = args
        .cell_id
        .into_iter()
        .map(|cell_id| resolve_explicit_or_reused_cell_id(&ledger, Some(cell_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let remove_deps = args
        .dependency
        .into_iter()
        .map(|cell_id| resolve_explicit_or_reused_cell_id(&ledger, Some(cell_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    for cell_id in dependents {
        let mut cell = ledger.load_cell_for_id(&cell_id)?;
        let deps = cell.dependencies_mut();

        deps.retain(|dep| {
            if args.all || remove_deps.contains(dep) {
                info!("removing dependency from {}: {}", cell_id, dep);
                false
            } else {
                true
            }
        });

        ledger.save_cell(&cell)?;
    }

    Ok(())
}
