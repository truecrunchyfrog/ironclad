mod audit;
mod fact;
mod catalog;
mod dependency;
mod operation;
mod review;
mod schema;

use crate::{args::Command, config::Config};

pub(super) fn dispatch(config: &Config, command: Command) -> anyhow::Result<()> {
    match command {
        Command::Catalog(cmd) => catalog::dispatch(config, cmd),
        Command::Fact(cmd) => fact::dispatch(config, cmd),
        Command::Dependency(cmd) => dependency::dispatch(config, cmd),
        Command::Schema(cmd) => schema::dispatch(config, cmd),
        Command::Operation(cmd) => operation::dispatch(config, cmd),
        Command::Audit(args) => audit::dispatch(config, args),
        Command::Review(args) => review::dispatch(config, args),
    }
}
