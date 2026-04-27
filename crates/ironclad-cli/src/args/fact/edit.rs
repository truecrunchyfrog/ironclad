use clap::Args;

/// Open a fact in your editor.
#[derive(Args)]
#[command(long_about = "Open a fact in `$EDITOR`.\n\n\
`edit` resolves a fact selector to one fact file and opens that file in your \
configured editor.\n\n\
Use a label for normal indexed facts or a fact ID when working directly with an \
unindexed fact.")]
pub(crate) struct EditFactArgs {
    /// Fact selector to edit.
    pub(crate) selector: String,
}
