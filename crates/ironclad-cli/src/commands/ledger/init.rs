use std::env::current_dir;

use rnacl_core::ledger::Ledger;

use crate::args::ledger::init::InitLedgerArgs;

pub(super) fn dispatch(args: InitLedgerArgs) -> anyhow::Result<()> {
    let dir = args.dir.unwrap_or(current_dir()?);
    Ledger::create_ledger(&dir)?;
    Ok(())
}
