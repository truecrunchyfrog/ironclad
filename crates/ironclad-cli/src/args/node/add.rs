use clap::Args;

/// Create a node.
#[derive(Args)]
pub(crate) struct AddNodeArgs {
    /// Choose the new node's ID instead of generating one.
    #[arg(long)]
    pub(crate) id: Option<String>,

    /// Describe the node.
    #[arg(long)]
    pub(crate) description: Option<String>,

    /// Skip automatic node reuse (see `node use`).
    #[arg(long)]
    pub(crate) no_use: bool,
}
