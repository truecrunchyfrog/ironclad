use clap::{ArgGroup, Args};

/// Create a cell.
#[derive(Args)]
#[command(group(
    ArgGroup::new("id_kind")
        .args(["cell_id", "generate_id"])
        .required(true)
        .multiple(false)
))]
pub(crate) struct AddCellArgs {
    /// An ID of the cell to create.
    pub(crate) cell_id: Option<String>,

    /// Generate an ID instead of specifying one.
    #[arg(long)]
    pub(crate) generate_id: bool,

    /// Describe the cell.
    #[arg(long)]
    pub(crate) description: Option<String>,

    /// Skip automatic cell reuse (see `cell use`).
    #[arg(long)]
    pub(crate) no_use: bool,
}
