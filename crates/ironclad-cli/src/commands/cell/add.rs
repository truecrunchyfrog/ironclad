use std::time::Duration;

use ironclad_core::cell::{Cell, id::CellId};

use crate::{args::cell::add::AddCellArgs, config::Config, helper::resolve_cluster, reuse_cell};

pub(super) fn dispatch(_config: &Config, args: AddCellArgs) -> anyhow::Result<()> {
    let cell_id: CellId = match args {
        AddCellArgs {
            cell_id: Some(cell_id),
            generate_id: false,
            ..
        } => cell_id.into(),
        AddCellArgs {
            cell_id: None,
            generate_id: true,
            ..
        } => Default::default(),
        _ => unreachable!(),
    };

    let cell = Cell::new(
        cell_id,
        args.description,
        Default::default(),
        args.cache_lifespan
            .map_or(Duration::ZERO, std::convert::Into::into),
        Default::default(),
    );

    let cluster = resolve_cluster()?;
    cluster.add_cell(&cell)?;

    println!("{}", cell.id());

    if !args.no_use {
        reuse_cell::set(&cluster, cell.id().clone(), None)?;
    }

    Ok(())
}
