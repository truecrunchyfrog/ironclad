use clap::Args;
use clap_stdin::FileOrStdout;

/// Capture a snapshot of currently evaluated state.
#[derive(Args)]
pub(crate) struct ResolveArgs {

    /// File to write snapshot to.
    #[arg(short, long)]
    pub(crate) destination: Option<FileOrStdout>,
}
