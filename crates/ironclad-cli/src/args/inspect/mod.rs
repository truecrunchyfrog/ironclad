use clap::Args;
use clap_stdin::FileOrStdin;

/// Show a snapshot.
#[derive(Args)]
pub(crate) struct InspectArgs {
    /// ID of fact whose samples to inspect.
    pub(crate) fact_id: Option<String>,

    /// Index of sample to show (starting from 1).
    pub(crate) index: Option<usize>,

    /// Show trace.
    #[arg(short, long, requires = "fact_id")]
    pub(crate) trace: bool,

    /// Snapshot to inspect (default: catalog's baseline).
    #[arg(short, long)]
    pub(crate) snapshot: Option<FileOrStdin>,

    /// Print the snapshot in its JSON format.
    #[arg(short, long, conflicts_with = "fact_id")]
    pub(crate) raw: bool,
}
