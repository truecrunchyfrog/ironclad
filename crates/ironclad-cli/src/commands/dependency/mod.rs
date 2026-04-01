mod add;
mod remove;

use crate::{args::dependency::DependencyCommand, config::Config};

pub(super) fn dispatch(config: &Config, command: DependencyCommand) -> anyhow::Result<()> {
    match command {
        DependencyCommand::Add(args) => add::dispatch(config, args),
        DependencyCommand::Remove(args) => remove::dispatch(config, args),
    }
}
