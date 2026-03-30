use crate::{
    args::cell::edit::EditCellArgs,
    helper::{resolve_explicit_or_reused_cell, resolve_ledger},
};

pub(super) fn dispatch(args: EditCellArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let mut cell = resolve_explicit_or_reused_cell(&ledger, args.cell_id)?;

    if let Some(description) = args.description {
        *cell.description_mut() = Some(description);
    }

    if args.unset_description {
        *cell.description_mut() = None;
    }

    ledger.save_cell(&cell)?;

    println!("{}", cell.id());

    Ok(())
}
