use clap::Args;

/// Remove a cell.
#[derive(Args)]
pub(crate) struct RemoveCellArgs {
    /// ID of cell to remove.
    pub(crate) cell_id: Option<String>,
}
