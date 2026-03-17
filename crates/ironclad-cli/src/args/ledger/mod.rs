pub(crate) mod init;

use clap::Subcommand;

use crate::args::ledger::init::InitLedgerArgs;

#[derive(Subcommand)]
pub(crate) enum LedgerCommand {
    Init(InitLedgerArgs),
}
