use clap::Args;

/// List a pipeline's stages.
#[derive(Args)]
pub(crate) struct ListPipelineArgs {
    /// ID of node to show pipeline of.
    pub(crate) node_id: Option<String>,

    /// Show the pipeline's raw JSON array.
    #[arg(short, long)]
    pub(crate) raw: bool,
}
