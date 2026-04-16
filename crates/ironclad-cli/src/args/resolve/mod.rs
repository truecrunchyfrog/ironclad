use clap::Args;
use clap_stdin::FileOrStdout;

/// Capture a snapshot of currently evaluated state.
#[derive(Args)]
pub(crate) struct ResolveArgs {
    // /// ID of fact to include in snapshot.
    // #[arg(short, long)]
    // pub(crate) include: Vec<String>,

    // /// ID of fact to exclude in snapshot.
    // #[arg(short, long, conflicts_with = "include")]
    // pub(crate) exclude: Vec<String>,
    /// File to write snapshot to (default: catalog's candidate).
    #[arg(short, long)]
    pub(crate) destination: Option<FileOrStdout>,
}
