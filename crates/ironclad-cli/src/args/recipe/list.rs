use clap::Args;

/// List a recipe's steps.
#[derive(Args)]
pub(crate) struct ListRecipeArgs {
    /// Label of fact to show recipe of.
    pub(crate) label: String,

    /// Show the recipe's raw JSON array.
    #[arg(short, long)]
    pub(crate) raw: bool,
}
