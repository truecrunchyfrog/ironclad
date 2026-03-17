use clap::Args;

/// Select a node, allowing you to omit the node ID in the following commands.
#[derive(Args)]
pub(crate) struct ReuseNodeArgs {
    pub(crate) node_id: Option<String>,

    /// Unset.
    #[arg(short, long, conflicts_with = "node_id")]
    pub(crate) unset: bool,

    /// Time until reuse expires.
    #[arg(short, long, requires = "node_id")]
    pub(crate) duration: Option<humantime::Duration>,
}
