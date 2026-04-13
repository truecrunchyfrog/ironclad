use clap::Args;

/// List a recipe's steps.
#[derive(Args)]
pub(crate) struct ListRecipeArgs {
    /// ID of fact to show recipe of.
    pub(crate) fact_id: String,

    /// Show the recipe's raw JSON array.
    #[arg(short, long)]
    pub(crate) raw: bool,
}
