use clap::{ArgGroup, Args};
use clap_stdin::{FileOrStdin, FileOrStdout};
use ironclad_core::fact::id::FactId;

/// Upgrade a snapshot with batches.
#[derive(Args)]
#[command(group(
    ArgGroup::new("selection")
        .args(["fact_id", "all"])
        .required(true)
))]
pub(crate) struct ApplyArgs {
    /// ID of fact to apply.
    pub(crate) fact_id: Vec<FactId>,

    /// Apply everything.
    #[arg(short, long)]
    pub(crate) all: bool,

    /// Snapshot to be promoted (default: catalog's candidate).
    #[arg(short, long)]
    pub(crate) promotion: Option<FileOrStdin>,

    /// Snapshot which to apply the promotion into (default: catalog's baseline).
    #[arg(short, long)]
    pub(crate) baseline: Option<FileOrStdin>,

    /// File to write snapshot to (default: catalog's baseline).
    #[arg(short, long)]
    pub(crate) destination: Option<FileOrStdout>,
}
