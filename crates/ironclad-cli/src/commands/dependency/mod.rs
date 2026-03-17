mod add;
mod remove;

use crate::args::dependency::DependencyCommand;

pub(super) fn dispatch(command: DependencyCommand) -> anyhow::Result<()> {
    match command {
        DependencyCommand::Add(args) => add::dispatch(args),
        DependencyCommand::Remove(args) => remove::dispatch(args),
    }
}
