pub(crate) mod init;

use clap::Subcommand;

use crate::args::cluster::init::InitClusterArgs;

#[derive(Subcommand)]
pub(crate) enum ClusterCommand {
    Init(InitClusterArgs),
}
