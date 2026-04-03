use clap::Args;

/// Remove a stage from a schema.
#[derive(Args)]
pub(crate) struct PopSchemaArgs {
    /// ID of cell.
    pub(crate) cell_id: Option<String>,

    /// Remove at a position instead of at the end.
    #[arg(short, long)]
    pub(crate) index: Option<usize>,

    /// Remove all stages.
    #[arg(long, conflicts_with = "index")]
    pub(crate) all: bool,
}
