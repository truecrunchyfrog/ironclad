use clap::Args;

/// Open a fact in $EDITOR.
#[derive(Args)]
pub(crate) struct EditFactArgs {
    /// Fact to edit.
    pub(crate) label: String,
}
