pub(crate) mod eval;
pub(crate) mod list;
pub(crate) mod pop;
pub(crate) mod push;

use clap::Subcommand;

use crate::args::recipe::{
    eval::EvalRecipeArgs, list::ListRecipeArgs, pop::PopRecipeArgs, push::PushRecipeArgs,
};

/// View, update, and evaluate the recipe of a fact.
#[derive(Subcommand)]
pub(crate) enum RecipeCommand {
    Add(PushRecipeArgs),
    Remove(PopRecipeArgs),
    Eval(EvalRecipeArgs),
    List(ListRecipeArgs),
}
