use clap::Args;

/// Edit a fact.
#[derive(Args)]
pub(crate) struct EditFactArgs {
    /// Fact to edit.
    pub(crate) label: String,

    /// Open fact in $EDITOR.
    #[arg(short, long)]
    pub(crate) raw: bool,

    /// Reassign the fact to a new label.
    #[arg(long)]
    pub(crate) rename: Option<String>,

    /// Change the description.
    #[arg(long)]
    pub(crate) description: Option<String>,

    /// Remove the description.
    #[arg(long, conflicts_with = "description")]
    pub(crate) unset_description: bool,

    /// Redact sample content when writing to a snapshot.
    #[arg(long)]
    pub(crate) secret: bool,

    /// Don't redact sample content when writing to a snapshot.
    #[arg(long, conflicts_with = "secret")]
    pub(crate) no_secret: bool,
}
