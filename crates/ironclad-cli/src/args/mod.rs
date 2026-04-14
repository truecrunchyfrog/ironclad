pub(crate) mod catalog;
pub(crate) mod fact;
pub(crate) mod operation;
pub(crate) mod recipe;
pub(crate) mod resolve;
pub(crate) mod review;

use clap::{Parser, Subcommand};

use crate::{
    args::{
        catalog::init::InitCatalogArgs,
        fact::{
            add::AddFactArgs, edit::EditFactArgs, list::ListFactArgs, remove::RemoveFactArgs,
            show::ShowFactArgs,
        },
        operation::OperationCommand,
        recipe::RecipeCommand,
        resolve::ResolveArgs,
        review::ReviewArgs,
    },
    config::Config,
};

#[derive(Parser)]
pub(crate) struct Cli {
    #[command(flatten)]
    pub(crate) config: Config,

    #[command(subcommand)]
    pub(crate) command: Command,
}

pub(crate) fn parse() -> Cli {
    Cli::parse()
}

#[derive(Subcommand)]
pub(crate) enum Command {
    Init(InitCatalogArgs),

    Add(AddFactArgs),
    Edit(EditFactArgs),
    #[command(alias = "rm")]
    Remove(RemoveFactArgs),
    #[command(alias = "sh")]
    Show(ShowFactArgs),
    #[command(alias = "ls")]
    List(ListFactArgs),

    #[command(subcommand, alias = "op")]
    Operation(OperationCommand),

    #[command(subcommand, name = "step")]
    Recipe(RecipeCommand),

    Resolve(ResolveArgs),

    Review(ReviewArgs),
}
