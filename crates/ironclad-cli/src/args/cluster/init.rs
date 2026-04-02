use std::path::PathBuf;

use clap::Args;

/// Set up a cluster.
#[derive(Args)]
pub(crate) struct InitClusterArgs {
    /// Where to place cluster directory.
    #[arg(long)]
    pub(crate) dir: Option<PathBuf>,
}
