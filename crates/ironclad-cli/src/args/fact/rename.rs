use clap::Args;

/// Rename a fact.
#[derive(Args)]
pub(crate) struct RenameFactArgs {
    /// Fact label or ID to rename.
    pub(crate) selector: String,

    /// The new label.
    pub(crate) new_label: String,
}
