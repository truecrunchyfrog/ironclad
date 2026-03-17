use std::path::PathBuf;

use clap::Args;

/// Set up a ledger.
#[derive(Args)]
pub(crate) struct InitLedgerArgs {
    /// Where to place ledger directory.
    #[arg(long)]
    pub(crate) dir: Option<PathBuf>,
}
