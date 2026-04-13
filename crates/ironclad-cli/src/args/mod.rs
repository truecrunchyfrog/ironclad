pub(crate) mod audit;
pub(crate) mod catalog;
pub(crate) mod fact;
pub(crate) mod operation;
pub(crate) mod recipe;
pub(crate) mod review;

use clap::{Parser, Subcommand};

use crate::{
    args::{
        audit::AuditArgs, catalog::CatalogCommand, fact::FactCommand, operation::OperationCommand,
        recipe::RecipeCommand, review::ReviewArgs,
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
    #[command(subcommand)]
    Catalog(CatalogCommand),

    #[command(subcommand)]
    Fact(FactCommand),

    #[command(subcommand, name = "op")]
    Operation(OperationCommand),

    #[command(subcommand)]
    Recipe(RecipeCommand),

    Audit(AuditArgs),

    Review(ReviewArgs),
}
