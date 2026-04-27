use clap::Args;
use clap_stdin::FileOrStdin;

/// Inspect a snapshot.
#[derive(Args)]
#[command(long_about = "Show a snapshot.\n\n\
Without a label, `inspect` prints one overview line per fact in the snapshot. \
With a label, it prints structured sample details for that fact.\n\n\
By default `inspect` reads the approved snapshot from the catalog, but \
`--snapshot` lets you read from another file or from stdin. `--raw` prints the \
underlying JSON format instead of the structured view.")]
pub(crate) struct InspectArgs {
    /// Fact label to inspect in detail.
    pub(crate) label: Option<String>,

    /// Include traces in the detailed view.
    #[arg(short, long, requires = "label")]
    pub(crate) trace: bool,

    /// Snapshot file to inspect.
    #[arg(short, long)]
    pub(crate) snapshot: Option<FileOrStdin>,

    /// Print the snapshot as JSON.
    #[arg(short, long, conflicts_with = "label")]
    pub(crate) raw: bool,
}
