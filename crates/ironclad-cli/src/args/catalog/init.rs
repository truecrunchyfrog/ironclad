use std::path::PathBuf;

use clap::Args;

/// Create a catalog directory.
#[derive(Args)]
#[command(long_about = "Set up a catalog.\n\n\
`init` creates the `.ironclad/` directory together with the standard layout: \
`facts/`, `index.toml`, and `snapshots/`.\n\n\
If you omit `--dir`, the catalog directory is created in the current working \
directory. If you pass `--dir`, Ironclad creates the catalog there. A path \
ending in `.ironclad` is treated as the exact catalog directory path.")]
pub(crate) struct InitCatalogArgs {
    /// Directory where the catalog should be created.
    #[arg(long)]
    pub(crate) dir: Option<PathBuf>,
}
