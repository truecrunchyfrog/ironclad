mod ack;
mod audit;
mod cell;
mod cluster;
mod dependency;
mod operation;
mod pipeline;

use crate::{args::Command, config::Config};

pub(super) fn dispatch(config: &Config, command: Command) -> anyhow::Result<()> {
    match command {
        Command::Cluster(cmd) => cluster::dispatch(config, cmd),
        Command::Cell(cmd) => cell::dispatch(config, cmd),
        Command::Dependency(cmd) => dependency::dispatch(config, cmd),
        Command::Pipeline(cmd) => pipeline::dispatch(config, cmd),
        Command::Operation(cmd) => operation::dispatch(config, cmd),
        Command::Audit(args) => audit::dispatch(config, args),
        Command::Ack(args) => ack::dispatch(config, args),
    }
}
