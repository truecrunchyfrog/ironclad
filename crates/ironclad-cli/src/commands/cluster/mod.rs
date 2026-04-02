mod init;

use crate::{args::cluster::ClusterCommand, config::Config};

pub(super) fn dispatch(config: &Config, command: ClusterCommand) -> anyhow::Result<()> {
    match command {
        ClusterCommand::Init(args) => init::dispatch(config, args),
    }
}
