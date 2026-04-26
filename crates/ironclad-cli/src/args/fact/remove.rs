use clap::Args;

/// Remove a fact.
#[derive(Args)]
pub(crate) struct RemoveFactArgs {
    /// Fact label or ID to remove.
    pub(crate) selector: String,
}
