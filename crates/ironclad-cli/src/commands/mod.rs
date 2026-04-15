mod apply;
mod catalog;
mod fact;
mod inspect;
mod operation;
mod recipe;
mod resolve;

use crate::{args::Command, config::Config};

pub(super) fn dispatch(config: &Config, command: Command) -> anyhow::Result<()> {
    match command {
        Command::Init(args) => catalog::init::dispatch(config, args),
        Command::Add(args) => fact::add::dispatch(config, args),
        Command::Edit(args) => fact::edit::dispatch(config, args),
        Command::Remove(args) => fact::remove::dispatch(config, args),
        Command::Show(args) => fact::show::dispatch(config, args),
        Command::List(args) => fact::list::dispatch(config, args),
        Command::Recipe(cmd) => recipe::dispatch(config, cmd),
        Command::Operation(cmd) => operation::dispatch(config, cmd),
        Command::Resolve(args) => resolve::dispatch(config, args),
        Command::Inspect(args) => inspect::dispatch(config, args),
        Command::Apply(args) => apply::dispatch(config, args),
    }
}
