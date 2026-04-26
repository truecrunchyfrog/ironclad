mod eval;
mod list;

use crate::{args::operation::OperationCommand, context::Context};

pub(super) fn dispatch(context: &Context, command: OperationCommand) -> anyhow::Result<()> {
    match command {
        OperationCommand::Eval(args) => eval::dispatch(context, args),
        OperationCommand::List(args) => list::dispatch(context, args),
    }
}
