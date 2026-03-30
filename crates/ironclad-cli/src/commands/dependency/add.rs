use log::info;

use crate::{
    args::dependency::add::AddDependencyArgs,
    helper::{resolve_explicit_or_reused_cell, resolve_explicit_or_reused_cell_id, resolve_ledger},
};

pub(super) fn dispatch(args: AddDependencyArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;

    let dependents = args
        .cell_id
        .into_iter()
        .map(|cell_id| resolve_explicit_or_reused_cell_id(&ledger, Some(cell_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let new_dependencies = {
        let mut result = Vec::new();

        for cell_id in args.dependency {
            result.push(resolve_explicit_or_reused_cell_id(&ledger, Some(cell_id))?);
        }

        for cell_id in args.from {
            let cell = resolve_explicit_or_reused_cell(&ledger, Some(cell_id))?;
            result.extend(cell.dependencies().to_owned());
        }

        if args.mirror {
            result.extend(dependents.to_owned());
        }

        result
    };

    for cell_id in dependents {
        let mut cell = ledger.load_cell_for_id(&cell_id)?;
        let deps = cell.dependencies_mut();

        for new_dependency in &new_dependencies {
            if new_dependency != &cell_id && !deps.contains(new_dependency) {
                info!("adding dependency to {}: {}", cell_id, new_dependency);
                deps.push(new_dependency.clone());
            }
        }

        ledger.save_cell(&cell)?;
    }

    Ok(())
}
