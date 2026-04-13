use clap::Args;

/// List dependencies.
#[derive(Args)]
pub(crate) struct ListDependencyArgs {
    /// ID of fact to list dependencies or dependents of.
    #[arg(default_value = "-")]
    pub(crate) fact_id: Vec<String>,

    /// List facts dependent on the fact instead of the opposite.
    #[arg(short)]
    pub(crate) invert: bool,

    /// List all facts.
    #[arg(short, long, conflicts_with = "fact_id")]
    pub(crate) all: bool,

    /// Skip facts when dependencies or dependents are empty.
    #[arg(long)]
    pub(crate) skip_empty: bool,
}
