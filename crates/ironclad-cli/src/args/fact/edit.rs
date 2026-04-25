use clap::Args;

/// Edit a fact.
#[derive(Args)]
pub(crate) struct EditFactArgs {
    /// Fact to edit.
    pub(crate) label: String,

    /// Reassign the fact to a new label.
    #[arg(long)]
    pub(crate) rename: Option<String>,
}
