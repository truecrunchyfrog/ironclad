use clap::Args;

/// Remove a fact.
///
/// `remove` deletes the fact file. If the fact has a label in `index.toml`, that
/// label is removed as well.
///
/// The selector can be either a label or a fact ID.
#[derive(Args)]
pub(crate) struct RemoveFactArgs {
    /// Fact selector to remove.
    pub(crate) selector: String,
}
