mod add;
mod edit;
mod list;
mod remove;
mod show;

use crate::{args::fact::FactCommand, config::Config};

pub(super) fn dispatch(config: &Config, command: FactCommand) -> anyhow::Result<()> {
    match command {
        FactCommand::Add(args) => add::dispatch(config, args),
        FactCommand::Edit(args) => edit::dispatch(config, args),
        FactCommand::Remove(args) => remove::dispatch(config, args),
        FactCommand::List(args) => list::dispatch(config, args),
        FactCommand::Show(args) => show::dispatch(config, args),
    }
}
