use std::path::PathBuf;

use clap::Args;

/// Set up a catalog.
#[derive(Args)]
pub(crate) struct InitCatalogArgs {
    /// Where to place catalog directory.
    #[arg(long)]
    pub(crate) dir: Option<PathBuf>,
}
