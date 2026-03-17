mod init;

use crate::args::ledger::LedgerCommand;

pub(super) fn dispatch(command: LedgerCommand) -> anyhow::Result<()> {
    match command {
        LedgerCommand::Init(args) => init::dispatch(args),
    }
}
