use clap::Args;

/// Open a fact in $EDITOR.
#[derive(Args)]
pub(crate) struct EditFactArgs {
    /// Fact label or ID to edit.
    pub(crate) selector: String,
}
