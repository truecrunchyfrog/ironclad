mod add;
mod edit;
mod list;
mod remove;
mod reuse;
mod show;

use crate::args::node::NodeCommand;

pub(super) fn dispatch(command: NodeCommand) -> anyhow::Result<()> {
    match command {
        NodeCommand::Reuse(args) => reuse::dispatch(args),
        NodeCommand::Add(args) => add::dispatch(args),
        NodeCommand::Edit(args) => edit::dispatch(args),
        NodeCommand::Remove(args) => remove::dispatch(args),
        NodeCommand::List(args) => list::dispatch(args),
        NodeCommand::Show(args) => show::dispatch(args),
    }
}
