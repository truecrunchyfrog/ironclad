mod add;
mod edit;
mod list;
mod remove;
mod reuse;
mod show;

use crate::args::cell::CellCommand;

pub(super) fn dispatch(command: CellCommand) -> anyhow::Result<()> {
    match command {
        CellCommand::Reuse(args) => reuse::dispatch(args),
        CellCommand::Add(args) => add::dispatch(args),
        CellCommand::Edit(args) => edit::dispatch(args),
        CellCommand::Remove(args) => remove::dispatch(args),
        CellCommand::List(args) => list::dispatch(args),
        CellCommand::Show(args) => show::dispatch(args),
    }
}
