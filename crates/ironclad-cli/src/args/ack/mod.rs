use clap::Args;

/// Add samples to the baseline.
#[derive(Args)]
pub(crate) struct AckArgs {
    /// ID of cell to ack.
    pub(crate) cell_id: Vec<String>,

    /// ID of dependency cell to ack, instead of acking the cell itself.
    #[arg(short, long)]
    pub(crate) dependency: Vec<String>,

    /// Ack all samples instead of selecting interactively.
    #[arg(short, long)]
    pub(crate) all: bool,
}
