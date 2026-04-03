mod eval;
mod list;
mod pop;
mod push;

use crate::{args::schema::SchemaCommand, config::Config};

pub(super) fn dispatch(config: &Config, command: SchemaCommand) -> anyhow::Result<()> {
    match command {
        SchemaCommand::Push(args) => push::dispatch(config, args),
        SchemaCommand::Pop(args) => pop::dispatch(config, args),
        SchemaCommand::Eval(args) => eval::dispatch(config, args),
        SchemaCommand::List(args) => list::dispatch(config, args),
    }
}
