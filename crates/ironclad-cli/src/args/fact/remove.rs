use clap::Args;

/// Remove a fact.
#[derive(Args)]
pub(crate) struct RemoveFactArgs {
    /// Fact to remove.
    pub(crate) label: String,
}
