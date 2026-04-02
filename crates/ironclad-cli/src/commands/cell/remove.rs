use crate::{
    args::cell::remove::RemoveCellArgs,
    config::Config,
    helper::{resolve_explicit_or_reused_cell_id, resolve_ledger},
};

pub(super) fn dispatch(_config: &Config, args: RemoveCellArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let cell_id = resolve_explicit_or_reused_cell_id(&ledger, args.cell_id)?;
    ledger.remove_cell(&cell_id)?;

    println!("{cell_id}");

    Ok(())
}
