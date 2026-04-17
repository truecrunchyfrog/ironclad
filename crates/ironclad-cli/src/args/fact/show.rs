use clap::Args;

/// Show details about a fact.
#[derive(Args)]
pub(crate) struct ShowFactArgs {
    /// Fact to show.
    pub(crate) label: String,

    /// Show the fact's raw JSON object.
    #[arg(short, long)]
    pub(crate) raw: bool,

    /// Show the fact's path.
    #[arg(short, long, conflicts_with = "raw")]
    pub(crate) path: bool,
}
