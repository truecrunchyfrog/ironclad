use clap::Args;

/// Remove a stage from a pipeline.
#[derive(Args)]
pub(crate) struct PopPipelineArgs {
    /// ID of node.
    pub(crate) node_id: Option<String>,

    /// Remove at a position instead of at the end.
    #[arg(short, long)]
    pub(crate) index: Option<usize>,

    /// Remove all stages.
    #[arg(long, conflicts_with = "index")]
    pub(crate) all: bool,
}
