mod init;

use crate::{args::ledger::LedgerCommand, config::Config};

pub(super) fn dispatch(config: &Config, command: LedgerCommand) -> anyhow::Result<()> {
    match command {
        LedgerCommand::Init(args) => init::dispatch(config, args),
    }
}
