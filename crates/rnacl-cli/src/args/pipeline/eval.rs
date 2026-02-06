use clap::Args;

/// Evaluate a pipeline.
#[derive(Args)]
pub(crate) struct EvalPipelineArgs {
    /// ID of node.
    pub(crate) node_id: Option<String>,

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
