use clap::Args;

/// Open a fact in your editor.
///
/// `edit` resolves a fact selector to one fact file and opens that file in your
/// configured editor.
///
/// Use a label for normal indexed facts or a fact ID when working directly with an
/// unindexed fact.
#[derive(Args)]
pub(crate) struct EditFactArgs {
    /// Fact selector to edit.
    pub(crate) selector: String,
}
