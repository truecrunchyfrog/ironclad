use clap::{ArgGroup, Args};

/// Show details about a fact.
#[derive(Args)]
#[command(group(
    ArgGroup::new("display")
        .args(["raw", "path"])
        .multiple(false)
))]
pub(crate) struct ShowFactArgs {
    /// ID of fact to show.
    pub(crate) fact_id: Option<String>,

    /// Show the fact's raw JSON object.
    #[arg(short, long)]
    pub(crate) raw: bool,

    /// Show the fact's path.
    #[arg(short, long)]
    pub(crate) path: bool,
}
