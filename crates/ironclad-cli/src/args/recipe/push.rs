use clap::Args;
use clap_stdin::MaybeStdin;

/// Add a step to a recipe.
#[derive(Args)]
pub(crate) struct PushRecipeArgs {
    /// ID of fact.
    pub(crate) fact_id: String,

    /// ID of operation.
    pub(crate) operation_id: String,

    /// Options to pass to the operation in JSON, or '-' to read from stdin.
    #[arg(short, long)]
    pub(crate) options: Option<MaybeStdin<String>>,

    /// Insert at a position instead of at the end.
    #[arg(short, long)]
    pub(crate) index: Option<usize>,
}
