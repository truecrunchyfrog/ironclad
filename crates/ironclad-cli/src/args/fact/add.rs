use clap::Args;

/// Create a fact.
#[derive(Args)]
pub(crate) struct AddFactArgs {
    /// An ID of the fact to create.
    pub(crate) fact_id: String,

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
