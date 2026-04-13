use clap::Args;

/// Evaluate a recipe.
#[derive(Args)]
pub(crate) struct EvalRecipeArgs {
    /// ID of fact.
    pub(crate) fact_id: String,

    /// Show the output of these steps.
    #[arg(short, long)]
    pub(crate) show: Option<Vec<usize>>,

    /// Show the output of all steps.
    #[arg(short = 'a', long, conflicts_with = "show")]
    pub(crate) show_all: bool,

    /// Only evaluate these steps.
    #[arg(short, long)]
    pub(crate) indices: Option<Vec<usize>>,
}
