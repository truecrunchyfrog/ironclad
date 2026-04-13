pub(crate) mod init;

use clap::Subcommand;

use crate::args::catalog::init::InitCatalogArgs;

/// Manage a catalog.
#[derive(Subcommand)]
pub(crate) enum CatalogCommand {
    Init(InitCatalogArgs),
}
