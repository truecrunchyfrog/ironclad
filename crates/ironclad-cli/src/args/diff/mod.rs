use clap::Args;
use clap_stdin::FileOrStdin;

/// Show difference between two snapshots.
#[derive(Args)]
pub(crate) struct DiffArgs {
    /// ID of fact whose samples to inspect.
    pub(crate) fact_id: Option<String>,

    /// Index of sample to show (starting from 1).
    pub(crate) index: Option<usize>,

    /// Show trace.
    #[arg(short, long)]
    pub(crate) trace: bool,

    /// Proposed snapshot (default: catalog's candidate).
    #[arg(short, long)]
    pub(crate) proposal: Option<FileOrStdin>,

    /// Older snapshot (default: catalog's baseline).
    #[arg(short, long)]
    pub(crate) baseline: Option<FileOrStdin>,

    /// Print the snapshot in its JSON format.
    #[arg(short, long, conflicts_with = "fact_id")]
    pub(crate) raw: bool,
}
