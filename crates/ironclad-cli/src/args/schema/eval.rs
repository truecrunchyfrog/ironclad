use clap::Args;

/// Evaluate a schema.
#[derive(Args)]
pub(crate) struct EvalSchemaArgs {
    /// ID of fact.
    pub(crate) fact_id: Option<String>,

    /// Show the output of these stages.
    #[arg(short, long)]
    pub(crate) show: Option<Vec<usize>>,

    /// Show the output of all stages.
    #[arg(short = 'a', long, conflicts_with = "show")]
    pub(crate) show_all: bool,

    /// Only evaluate these stages.
    #[arg(short, long)]
    pub(crate) indices: Option<Vec<usize>>,
}
