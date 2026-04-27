use clap::Args;
use clap_stdin::FileOrStdin;

/// Show comparison between two snapshots.
#[derive(Args)]
pub(crate) struct DiffArgs {
    /// Fact whose samples to inspect.
    pub(crate) label: Option<String>,

    /// Show trace.
    #[arg(short, long, requires = "label")]
    pub(crate) trace: bool,

    /// Proposed snapshot (default: catalog's actual).
    #[arg(short, long)]
    pub(crate) proposal: Option<FileOrStdin>,

    /// Baseline snapshot (default: catalog's canon).
    #[arg(short, long)]
    pub(crate) baseline: Option<FileOrStdin>,

    /// Print the snapshot in its JSON format.
    #[arg(short, long, conflicts_with = "label")]
    pub(crate) raw: bool,
}
