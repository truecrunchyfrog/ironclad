mod add;
mod edit;
mod list;
mod remove;
mod reuse;
mod show;

use crate::{args::cell::CellCommand, config::Config};

pub(super) fn dispatch(config: &Config, command: CellCommand) -> anyhow::Result<()> {
    match command {
        CellCommand::Reuse(args) => reuse::dispatch(config, args),
        CellCommand::Add(args) => add::dispatch(config, args),
        CellCommand::Edit(args) => edit::dispatch(config, args),
        CellCommand::Remove(args) => remove::dispatch(config, args),
        CellCommand::List(args) => list::dispatch(config, args),
        CellCommand::Show(args) => show::dispatch(config, args),
    }
}
