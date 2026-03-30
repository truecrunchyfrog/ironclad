use clap::{ArgGroup, Args};

/// Show details about a cell.
#[derive(Args)]
#[command(group(
    ArgGroup::new("display")
        .args(["raw", "path"])
        .multiple(false)
))]
pub(crate) struct ShowCellArgs {
    /// ID of cell to show.
    pub(crate) cell_id: Option<String>,

    /// Show the cell's raw JSON object.
    #[arg(short, long)]
    pub(crate) raw: bool,

    /// Show the cell's path.
    #[arg(short, long)]
    pub(crate) path: bool,
}
