mod ack;
mod audit;
mod dependency;
mod ledger;
mod node;
mod operation;
mod pipeline;

use crate::args::{Cli, Command};

pub(super) fn dispatch(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Command::Ledger(cmd) => ledger::dispatch(cmd),
        Command::Node(cmd) => node::dispatch(cmd),
        Command::Dependency(cmd) => dependency::dispatch(cmd),
        Command::Pipeline(cmd) => pipeline::dispatch(cmd),
        Command::Operation(cmd) => operation::dispatch(cmd),
        Command::Audit(args) => audit::dispatch(args),
        Command::Ack(args) => ack::dispatch(args),
    }
}
