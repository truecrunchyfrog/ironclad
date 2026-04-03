use clap::Args;

/// Review the audit and promote samples to baseline.
#[derive(Args)]
pub(crate) struct ReviewArgs {
    /// ID of cell to ack.
    pub(crate) cell_id: Vec<String>,

    /// ID of dependency cell to ack, instead of acking the cell itself.
    #[arg(short, long)]
    pub(crate) dependency: Vec<String>,

    /// Ack all samples instead of selecting interactively.
    #[arg(short, long)]
    pub(crate) all: bool,
}
