use clap::Args;

/// Rename a fact.
#[derive(Args)]
pub(crate) struct RenameFactArgs {
    /// Fact to rename.
    pub(crate) label: String,

    /// The new label.
    pub(crate) new_label: String,
}
