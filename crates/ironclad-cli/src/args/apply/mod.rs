use clap::{ArgGroup, Args};
use clap_stdin::{FileOrStdin, FileOrStdout};

/// Upgrade a snapshot with batches.
#[derive(Args)]
#[command(group(
    ArgGroup::new("selection")
        .args(["label", "all"])
        .required(true)
))]
pub(crate) struct ApplyArgs {
    /// Fact to apply.
    pub(crate) label: Vec<String>,

    /// Apply everything.
    #[arg(short, long)]
    pub(crate) all: bool,

    /// Snapshot to be promoted (default: catalog's actual).
    #[arg(short, long)]
    pub(crate) promotion: Option<FileOrStdin>,

    /// Snapshot which to apply the promotion into (default: catalog's canon).
    #[arg(short, long)]
    pub(crate) baseline: Option<FileOrStdin>,

    /// File to write snapshot to (default: catalog's canon).
    #[arg(short, long)]
    pub(crate) output: Option<FileOrStdout>,
}
