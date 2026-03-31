use std::time::Duration;

use ironclad_core::cell::Cell;

use crate::{args::cell::add::AddCellArgs, helper::resolve_ledger, reuse_cell};

pub(super) fn dispatch(args: AddCellArgs) -> anyhow::Result<()> {
    let cell = Cell::new(
        args.cell_id.map_or(Default::default(), |id| id.into()),
        args.description,
        Default::default(),
        Duration::from_hours(1),
        Default::default(),
    );

    let ledger = resolve_ledger()?;
    ledger.add_cell(&cell)?;

    println!("{}", cell.id());

    if !args.no_use {
        reuse_cell::set(&ledger, cell.id().clone(), None)?;
    }

    Ok(())
}
