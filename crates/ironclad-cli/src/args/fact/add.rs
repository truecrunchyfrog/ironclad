use clap::{ArgGroup, Args};

/// Create a fact file.
#[derive(Args)]
#[command(group(ArgGroup::new("indexing").args(["label", "no_index"]).required(true)))]
#[command(long_about = "Create a fact.\n\n\
`add` creates a new fact file under the catalog's `facts/` directory.\n\n\
If you pass a label, the fact is also added to `index.toml` and can be used by \
that label in later commands. If you pass `--no-index`, the fact is created \
without a label and must be addressed by fact ID.")]
pub(crate) struct AddFactArgs {
    /// Label to assign and add to the index.
    pub(crate) label: Option<String>,

    /// Create the fact without adding it to the index.
    #[arg(long)]
    pub(crate) no_index: bool,
}
