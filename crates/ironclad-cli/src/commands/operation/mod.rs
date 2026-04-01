mod eval;
mod list;

use crate::{args::operation::OperationCommand, config::Config};

pub(super) fn dispatch(config: &Config, command: OperationCommand) -> anyhow::Result<()> {
    match command {
        OperationCommand::Eval(args) => eval::dispatch(config, args),
        OperationCommand::List(args) => list::dispatch(config, args),
    }
}
