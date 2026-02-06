use log::info;

use crate::{
    args::dependency::remove::RemoveDependencyArgs,
    helper::{resolve_explicit_or_reused_node_id, resolve_ledger},
};

pub(super) fn dispatch(args: RemoveDependencyArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;

    let dependents = args
        .node_id
        .into_iter()
        .map(|node_id| resolve_explicit_or_reused_node_id(&ledger, Some(node_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let remove_deps = args
        .dependency
        .into_iter()
        .map(|node_id| resolve_explicit_or_reused_node_id(&ledger, Some(node_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    for node_id in dependents {
        let mut node = ledger.load_node_for_id(&node_id)?;
        let deps = node.dependencies_mut();

        deps.retain(|dep| {
            if args.all || remove_deps.contains(dep) {
                info!("removing dependency from {}: {}", node_id, dep);
                false
            } else {
                true
            }
        });

        ledger.save_node(&node)?;
    }

    Ok(())
}
