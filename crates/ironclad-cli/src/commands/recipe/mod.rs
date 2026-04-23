mod list;
mod pop;
mod push;

use crate::{args::recipe::RecipeCommand, config::Config};

pub(super) fn dispatch(config: &Config, command: RecipeCommand) -> anyhow::Result<()> {
    match command {
        RecipeCommand::Add(args) => push::dispatch(config, args),
        RecipeCommand::Remove(args) => pop::dispatch(config, args),
        RecipeCommand::List(args) => list::dispatch(config, args),
    }
}
