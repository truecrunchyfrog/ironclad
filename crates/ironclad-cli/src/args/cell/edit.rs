use clap::Args;

/// Edit a cell.
#[derive(Args)]
pub(crate) struct EditCellArgs {
    /// ID of cell to edit.
    pub(crate) cell_id: Option<String>,

    /// Change the description.
    #[arg(long)]
    pub(crate) description: Option<String>,

    /// Remove the description.
    #[arg(long, conflicts_with = "description")]
    pub(crate) unset_description: bool,

    /// Change the lifespan.
    #[arg(long)]
    pub(crate) lifespan: Option<humantime::Duration>,

    /// Remove the lifespan.
    #[arg(long, conflicts_with = "lifespan")]
    pub(crate) unset_lifespan: bool,
}
