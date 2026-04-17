use clap::Args;

/// Evaluate a recipe.
#[derive(Args)]
pub(crate) struct EvalRecipeArgs {
    /// Label of fact.
    pub(crate) label: String,

    /// Show the output of this step.
    #[arg(short, long)]
    pub(crate) show: Option<Vec<usize>>,

    /// Show the output of all steps.
    #[arg(short = 'a', long, conflicts_with = "show")]
    pub(crate) show_all: bool,

    /// Only evaluate this step.
    #[arg(short, long)]
    pub(crate) indices: Option<Vec<usize>>,
}
