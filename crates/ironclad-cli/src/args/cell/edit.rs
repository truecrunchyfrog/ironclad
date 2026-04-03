use clap::Args;

/// Edit a cell.
#[derive(Args)]
pub(crate) struct EditCellArgs {
    /// ID of cell to edit.
    pub(crate) cell_id: Option<String>,

    /// Change the cell's ID.
    #[arg(long)]
    pub(crate) id: Option<String>,

    /// Change the description.
    #[arg(long)]
    pub(crate) description: Option<String>,

    /// Remove the description.
    #[arg(long, conflicts_with = "description")]
    pub(crate) unset_description: bool,

    /// Change the cache lifespan.
    #[arg(long)]
    pub(crate) cache_lifespan: Option<humantime::Duration>,

    /// Remove the cache lifespan.
    #[arg(long, conflicts_with = "lifespan")]
    pub(crate) unset_cache_lifespan: bool,
}
