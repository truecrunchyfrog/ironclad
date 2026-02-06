use log::info;

use crate::{
    args::dependency::add::AddDependencyArgs,
    helper::{resolve_explicit_or_reused_node, resolve_explicit_or_reused_node_id, resolve_ledger},
};

pub(super) fn dispatch(args: AddDependencyArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;

    let dependents = args
        .node_id
        .into_iter()
        .map(|node_id| resolve_explicit_or_reused_node_id(&ledger, Some(node_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let new_dependencies = {
        let mut result = Vec::new();

        for node_id in args.dependency {
            result.push(resolve_explicit_or_reused_node_id(&ledger, Some(node_id))?);
        }

        for node_id in args.from {
            let node = resolve_explicit_or_reused_node(&ledger, Some(node_id))?;
            result.extend(node.dependencies().to_owned());
        }

        if args.mirror {
            result.extend(dependents.to_owned());
        }

        result
    };

    for node_id in dependents {
        let mut node = ledger.load_node_for_id(&node_id)?;
        let deps = node.dependencies_mut();

        for new_dependency in &new_dependencies {
            if new_dependency != &node_id && !deps.contains(new_dependency) {
                info!("adding dependency to {}: {}", node_id, new_dependency);
                deps.push(new_dependency.clone());
            }
        }

        ledger.save_node(&node)?;
    }

    Ok(())
}
