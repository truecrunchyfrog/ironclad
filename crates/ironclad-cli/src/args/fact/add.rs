use clap::{ArgGroup, Args};

/// Create a fact.
#[derive(Args)]
#[command(group(
    ArgGroup::new("id_kind")
        .args(["fact_id", "generate_id"])
        .required(true)
        .multiple(false)
))]
pub(crate) struct AddFactArgs {
    /// An ID of the fact to create.
    pub(crate) fact_id: Option<String>,

    /// Generate an ID instead of specifying one.
    #[arg(long)]
    pub(crate) generate_id: bool,

    /// Describe the fact's purpose.
    #[arg(long)]
    pub(crate) description: Option<String>,

    /// Specify the fact's cache lifespan for audits.
    #[arg(long)]
    pub(crate) cache_lifespan: Option<humantime::Duration>,

    /// Skip automatic fact reuse (see `fact use`).
    #[arg(long)]
    pub(crate) no_use: bool,
}
