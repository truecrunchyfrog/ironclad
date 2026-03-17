use clap::Args;

/// Add samples to the baseline.
#[derive(Args)]
pub(crate) struct AckArgs {
    /// ID of node to ack.
    pub(crate) node_id: Vec<String>,

    /// ID of dependency node to ack, instead of acking the node itself.
    #[arg(short, long)]
    pub(crate) dependency: Vec<String>,

    /// Ack all samples instead of selecting interactively.
    #[arg(short, long)]
    pub(crate) all: bool,
}
