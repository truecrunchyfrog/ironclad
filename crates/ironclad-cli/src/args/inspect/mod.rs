use clap::Args;
use clap_stdin::FileOrStdin;

/// Show a snapshot.
#[derive(Args)]
pub(crate) struct InspectArgs {
    /// Fact whose samples to inspect.
    pub(crate) label: Option<String>,

    /// Show trace.
    #[arg(short, long, requires = "label")]
    pub(crate) trace: bool,

    /// Snapshot to inspect (default: catalog's canon).
    #[arg(short, long)]
    pub(crate) snapshot: Option<FileOrStdin>,

    /// Print the snapshot in its JSON format.
    #[arg(short, long, conflicts_with = "label")]
    pub(crate) raw: bool,
}
