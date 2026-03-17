use clap::Args;

/// Add a stage to a pipeline.
#[derive(Args)]
pub(crate) struct PushPipelineArgs {
    /// ID of node.
    pub(crate) node_id: String,

    /// ID of operation.
    pub(crate) operation_id: String,

    /// Options to pass to the operation, in JSON.
    #[arg(short, long)]
    pub(crate) options: Option<String>,

    /// Insert at a position instead of at the end.
    #[arg(short, long)]
    pub(crate) index: Option<usize>,
}
