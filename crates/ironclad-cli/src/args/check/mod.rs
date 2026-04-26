use clap::Args;
use clap_stdin::FileOrStdin;

/// Ensure two snapshots are identical.
#[derive(Args)]
pub(crate) struct CheckArgs {
    /// Proposed snapshot (default: catalog's actual).
    #[arg(short, long)]
    pub(crate) proposal: Option<FileOrStdin>,

    /// Baseline snapshot (default: catalog's canon).
    #[arg(short, long)]
    pub(crate) baseline: Option<FileOrStdin>,
}
