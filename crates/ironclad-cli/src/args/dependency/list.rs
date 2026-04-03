use clap::Args;

/// List dependencies.
#[derive(Args)]
pub(crate) struct ListDependencyArgs {
    /// ID of cell to list dependencies or dependents of.
    #[arg(default_value = "-")]
    pub(crate) cell_id: Vec<String>,

    /// List cells dependent on the cell instead of the opposite.
    #[arg(short)]
    pub(crate) invert: bool,

    /// List all cells.
    #[arg(short, long, conflicts_with = "cell_id")]
    pub(crate) all: bool,

    /// Skip cells when dependencies or dependents are empty.
    #[arg(long)]
    pub(crate) skip_empty: bool,
}
