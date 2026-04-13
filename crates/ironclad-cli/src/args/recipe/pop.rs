use clap::Args;

/// Remove a step from a recipe.
#[derive(Args)]
pub(crate) struct PopRecipeArgs {
    /// ID of fact.
    pub(crate) fact_id: Option<String>,

    /// Remove at a position instead of at the end.
    #[arg(short, long)]
    pub(crate) index: Option<usize>,

    /// Remove all steps.
    #[arg(long, conflicts_with = "index")]
    pub(crate) all: bool,
}
