use clap::Args;

/// Remove a node.
#[derive(Args)]
pub(crate) struct RemoveNodeArgs {
    /// ID of node to remove.
    pub(crate) node_id: Option<String>,
}
