use clap::{ArgGroup, Args};

/// Show details about a node.
#[derive(Args)]
#[command(group(
    ArgGroup::new("display")
        .args(["raw", "path"])
        .multiple(false)
))]
pub(crate) struct ShowNodeArgs {
    /// ID of node to show.
    pub(crate) node_id: Option<String>,

    /// Show the node's raw JSON object.
    #[arg(short, long)]
    pub(crate) raw: bool,

    /// Show the node's path.
    #[arg(short, long)]
    pub(crate) path: bool,
}
