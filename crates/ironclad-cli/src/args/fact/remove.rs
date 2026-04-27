use clap::Args;

/// Remove a fact.
#[derive(Args)]
#[command(long_about = "Remove a fact.\n\n\
`remove` deletes the fact file. If the fact has a label in `index.toml`, that \
label is removed as well.\n\n\
The selector can be either a label or a fact ID.")]
pub(crate) struct RemoveFactArgs {
    /// Fact selector to remove.
    pub(crate) selector: String,
}
