use clap::Args;

/// Create a cell.
#[derive(Args)]
pub(crate) struct AddCellArgs {
    /// Choose the new cell's ID instead of generating one.
    pub(crate) cell_id: Option<String>,

    /// Describe the cell.
    #[arg(long)]
    pub(crate) description: Option<String>,

    /// Skip automatic cell reuse (see `cell use`).
    #[arg(long)]
    pub(crate) no_use: bool,
}
