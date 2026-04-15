use clap::Args;
use clap_stdin::{FileOrStdin, FileOrStdout};

/// Upgrade a snapshot with batches.
#[derive(Args)]
pub(crate) struct ApplyArgs {
    /// Snapshot to be promoted.
    #[arg(short, long)]
    pub(crate) promotion: Option<FileOrStdin>,

    /// Snapshot which to apply the promotion into.
    #[arg(short, long)]
    pub(crate) baseline: Option<FileOrStdin>,

    /// File to write snapshot to.
    #[arg(short, long)]
    pub(crate) destination: Option<FileOrStdout>,
}
