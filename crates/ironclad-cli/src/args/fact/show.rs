use clap::Args;

/// Show a fact.
#[derive(Args)]
pub(crate) struct ShowFactArgs {
    /// Fact label or ID to show.
    pub(crate) selector: String,

    /// Show the fact's path.
    #[arg(short, long)]
    pub(crate) path: bool,
}
