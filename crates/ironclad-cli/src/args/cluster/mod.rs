pub(crate) mod init;

use clap::Subcommand;

use crate::args::cluster::init::InitClusterArgs;

/// Manage a cluster.
#[derive(Subcommand)]
pub(crate) enum ClusterCommand {
    Init(InitClusterArgs),
}
