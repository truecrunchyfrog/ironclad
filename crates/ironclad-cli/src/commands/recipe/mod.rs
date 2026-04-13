mod eval;
mod list;
mod pop;
mod push;

use crate::{args::recipe::RecipeCommand, config::Config};

pub(super) fn dispatch(config: &Config, command: RecipeCommand) -> anyhow::Result<()> {
    match command {
        RecipeCommand::Push(args) => push::dispatch(config, args),
        RecipeCommand::Pop(args) => pop::dispatch(config, args),
        RecipeCommand::Eval(args) => eval::dispatch(config, args),
        RecipeCommand::List(args) => list::dispatch(config, args),
    }
}
