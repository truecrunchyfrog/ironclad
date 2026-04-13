mod audit;
mod catalog;
mod fact;
mod operation;
mod recipe;
mod review;

use crate::{args::Command, config::Config};

pub(super) fn dispatch(config: &Config, command: Command) -> anyhow::Result<()> {
    match command {
        Command::Catalog(cmd) => catalog::dispatch(config, cmd),
        Command::Fact(cmd) => fact::dispatch(config, cmd),
        Command::Recipe(cmd) => recipe::dispatch(config, cmd),
        Command::Operation(cmd) => operation::dispatch(config, cmd),
        Command::Audit(args) => audit::dispatch(config, args),
        Command::Review(args) => review::dispatch(config, args),
    }
}
