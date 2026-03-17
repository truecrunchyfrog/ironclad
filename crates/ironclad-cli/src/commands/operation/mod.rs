mod eval;
mod list;

use crate::args::operation::OperationCommand;

pub(super) fn dispatch(command: OperationCommand) -> anyhow::Result<()> {
    match command {
        OperationCommand::Eval(args) => eval::dispatch(args),
        OperationCommand::List(args) => list::dispatch(args),
    }
}
