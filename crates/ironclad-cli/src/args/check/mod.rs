use clap::Args;
use clap_stdin::FileOrStdin;

/// Ensure two snapshots are identical.
#[derive(Args)]
pub(crate) struct CheckArgs {
    /// Proposed snapshot (default: catalog's candidate).
    #[arg(short, long)]
    pub(crate) proposal: Option<FileOrStdin>,

    /// Older snapshot (default: catalog's baseline).
    #[arg(short, long)]
    pub(crate) baseline: Option<FileOrStdin>,
}
