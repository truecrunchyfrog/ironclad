use clap::Args;

/// Show details about a fact.
#[derive(Args)]
pub(crate) struct ShowFactArgs {
    /// Fact to show.
    pub(crate) label: String,

    /// Show the fact's path.
    #[arg(short, long)]
    pub(crate) path: bool,
}
