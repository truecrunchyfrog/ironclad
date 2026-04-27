use clap::Args;

/// Rename an indexed fact label.
#[derive(Args)]
#[command(long_about = "Rename a fact.\n\n\
`rename` changes the label recorded in `index.toml`. The underlying fact file \
and fact ID do not change.\n\n\
The selector can be either the current label or the fact ID.")]
pub(crate) struct RenameFactArgs {
    /// Fact selector to rename.
    pub(crate) selector: String,

    /// New label to store in the index.
    pub(crate) new_label: String,
}
