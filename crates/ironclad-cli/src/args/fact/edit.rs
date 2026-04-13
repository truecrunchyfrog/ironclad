use clap::Args;

/// Edit a fact.
#[derive(Args)]
pub(crate) struct EditFactArgs {
    /// ID of fact to edit.
    pub(crate) fact_id: String,

    /// Change the fact's ID.
    #[arg(long)]
    pub(crate) id: Option<String>,

    /// Change the description.
    #[arg(long)]
    pub(crate) description: Option<String>,

    /// Remove the description.
    #[arg(long, conflicts_with = "description")]
    pub(crate) unset_description: bool,

    /// Change the cache lifespan.
    #[arg(long)]
    pub(crate) cache_lifespan: Option<humantime::Duration>,

    /// Remove the cache lifespan.
    #[arg(long, conflicts_with = "cache_lifespan")]
    pub(crate) unset_cache_lifespan: bool,
}
