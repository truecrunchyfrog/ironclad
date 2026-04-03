use std::time::Duration;

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

    if let Some(lifespan) = args.lifespan {
        *cell.cache_lifespan_mut() = lifespan.into();
    }

    if args.unset_lifespan {
        *cell.cache_lifespan_mut() = Duration::ZERO;
    }

    cluster.save_cell(&cell)?;

    println!("{}", cell.id());

    Ok(())
}
