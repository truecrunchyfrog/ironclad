use clap::Args;

/// Select a fact, allowing you to omit the fact ID in the following commands.
#[derive(Args)]
pub(crate) struct ReuseFactArgs {
    pub(crate) fact_id: Option<String>,

    /// Unset.
    #[arg(short, long, conflicts_with = "fact_id")]
    pub(crate) unset: bool,

    /// Time until reuse expires.
    #[arg(short, long, requires = "fact_id")]
    pub(crate) duration: Option<humantime::Duration>,
}
