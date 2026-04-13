use clap::Args;

/// Remove a fact.
#[derive(Args)]
pub(crate) struct RemoveFactArgs {
    /// ID of fact to remove.
    pub(crate) fact_id: Option<String>,
}
