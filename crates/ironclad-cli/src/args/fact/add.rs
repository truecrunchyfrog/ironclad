use clap::{ArgGroup, Args};

/// Create a fact.
#[derive(Args)]
#[command(group(ArgGroup::new("indexing").args(["label", "no_index"]).required(true)))]
pub(crate) struct AddFactArgs {
    /// Assign a label to the fact.
    pub(crate) label: Option<String>,

    /// Don't index the fact.
    #[arg(long)]
    pub(crate) no_index: bool,

    /// Describe the fact's purpose.
    #[arg(long)]
    pub(crate) description: Option<String>,

    /// Redact sample content when writing to a snapshot.
    #[arg(long)]
    pub(crate) secret: bool,
}
