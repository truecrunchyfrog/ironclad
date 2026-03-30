use clap::Args;

/// Select a cell, allowing you to omit the cell ID in the following commands.
#[derive(Args)]
pub(crate) struct ReuseCellArgs {
    pub(crate) cell_id: Option<String>,

    /// Unset.
    #[arg(short, long, conflicts_with = "cell_id")]
    pub(crate) unset: bool,

    /// Time until reuse expires.
    #[arg(short, long, requires = "cell_id")]
    pub(crate) duration: Option<humantime::Duration>,
}
