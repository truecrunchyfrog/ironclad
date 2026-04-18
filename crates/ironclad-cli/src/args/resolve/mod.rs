use clap::Args;
use clap_stdin::FileOrStdout;

/// Capture a snapshot of currently evaluated state.
#[derive(Args)]
pub(crate) struct ResolveArgs {
    /// Include none except this fact in the snapshot.
    #[arg(short, long)]
    pub(crate) include: Vec<String>,

    /// Include all but this fact in the snapshot.
    #[arg(short = 'x', long, conflicts_with = "include")]
    pub(crate) exclude: Vec<String>,

    /// File to write snapshot to (default: catalog's candidate).
    #[arg(short, long)]
    pub(crate) output: Option<FileOrStdout>,

    /// Don't redact facts marked as secret.
    #[arg(long)]
    pub(crate) no_redact: bool,
}
