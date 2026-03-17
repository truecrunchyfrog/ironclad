use clap::Args;

/// Edit a node.
#[derive(Args)]
pub(crate) struct EditNodeArgs {
    /// ID of node to edit.
    pub(crate) node_id: Option<String>,

    /// Change the description.
    #[arg(long)]
    pub(crate) description: Option<String>,

    /// Remove the description.
    #[arg(long, conflicts_with = "description")]
    pub(crate) unset_description: bool,
}
