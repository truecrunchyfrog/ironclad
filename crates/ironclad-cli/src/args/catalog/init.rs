use std::path::PathBuf;

use clap::Args;

/// Create a catalog directory.
///
/// `init` creates the `.ironclad/` directory together with the standard layout:
/// `facts/`, `index.toml`, and `snapshots/`.
///
/// If you omit `--dir`, the catalog directory is created in the current working
/// directory. If you pass `--dir`, Ironclad creates the catalog there. A path
/// ending in `.ironclad` is treated as the exact catalog directory path.
#[derive(Args)]
pub(crate) struct InitCatalogArgs {
    /// Directory where the catalog should be created.
    #[arg(long)]
    pub(crate) dir: Option<PathBuf>,
}
