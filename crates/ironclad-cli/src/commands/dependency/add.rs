use log::info;

use crate::{
    args::dependency::add::AddDependencyArgs,
    config::Config,
    helper::{
        resolve_cluster, resolve_explicit_or_reused_cell, resolve_explicit_or_reused_cell_id,
    },
};

pub(super) fn dispatch(_config: &Config, args: AddDependencyArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;

    let dependents = args
        .cell_id
        .into_iter()
        .map(|cell_id| resolve_explicit_or_reused_cell_id(&cluster, Some(cell_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let new_dependencies = {
        let mut result = Vec::new();

        for cell_id in args.dependency {
            result.push(resolve_explicit_or_reused_cell_id(&cluster, Some(cell_id))?);
        }

        for cell_id in args.from {
            let cell = resolve_explicit_or_reused_cell(&cluster, Some(cell_id))?;
            result.extend(cell.dependencies().to_owned());
        }

        if args.mirror {
            result.extend(dependents.clone());
        }

        result
    };

    for cell_id in dependents {
        let mut cell = cluster.load_cell_for_id(&cell_id)?;
        let deps = cell.dependencies_mut();

        for new_dependency in &new_dependencies {
            if new_dependency != &cell_id && !deps.contains(new_dependency) {
                info!("adding dependency to {cell_id}: {new_dependency}");
                deps.push(new_dependency.clone());
            }
        }

        cluster.save_cell(&cell)?;
    }

    Ok(())
}
