use std::time::Duration;

use ironclad_core::cell::id::CellId;

use crate::{
    args::cell::edit::EditCellArgs,
    config::Config,
    helper::{resolve_cluster, resolve_explicit_or_reused_cell},
};

pub(super) fn dispatch(_config: &Config, args: EditCellArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;
    let mut cell = resolve_explicit_or_reused_cell(&cluster, args.cell_id)?;

    if let Some(description) = args.description {
        *cell.description_mut() = Some(description);
    }

    if args.unset_description {
        *cell.description_mut() = None;
    }

    if let Some(cache_lifespan) = args.cache_lifespan {
        *cell.cache_lifespan_mut() = cache_lifespan.into();
    }

    if args.unset_cache_lifespan {
        *cell.cache_lifespan_mut() = Duration::ZERO;
    }

    if let Some(new_id) = args.id {
        let old_id = cell.id();
        let new_id: CellId = new_id.into();

        cluster.remove_cell(old_id)?;
        *cell.id_mut() = new_id.clone();
        cluster.add_cell(&cell)?;

        for mut dependent_cell in cluster.load_cells()? {
            let dependencies = dependent_cell.dependencies_mut();
            if dependencies.contains(cell.id()) {
                *dependencies = dependencies
                    .iter()
                    .cloned()
                    .filter(|dependent_cell_id| dependent_cell_id != cell.id())
                    .collect();
                dependencies.push(new_id.clone());
                cluster.save_cell(&dependent_cell)?;
            }
        }
    } else {
        cluster.save_cell(&cell)?;
    }

    println!("{}", cell.id());

    Ok(())
}
