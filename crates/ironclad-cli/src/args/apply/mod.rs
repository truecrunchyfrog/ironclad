use clap::{ArgGroup, Args};
use clap_stdin::{FileOrStdin, FileOrStdout};

/// Apply changes into the approved snapshot.
#[derive(Args)]
#[command(group(
    ArgGroup::new("selection")
        .args(["label", "all"])
        .required(true)
))]
#[command(long_about = "Upgrade a snapshot with batches.\n\n\
`apply` promotes facts from the resolved snapshot into the approved snapshot. \
You can promote selected labels or replace the whole approved snapshot with \
`--all`.\n\n\
By default the resolved snapshot comes from `actual.json` and the approved \
snapshot comes from `canon.json`. Both can be overridden, and the result can be \
written to another file instead of replacing `canon.json`.")]
pub(crate) struct ApplyArgs {
    /// Fact labels to promote.
    pub(crate) label: Vec<String>,

    /// Replace the whole approved snapshot.
    #[arg(short, long)]
    pub(crate) all: bool,

    /// Resolved snapshot to promote from.
    #[arg(short, long)]
    pub(crate) promotion: Option<FileOrStdin>,

    /// Approved snapshot to update.
    #[arg(short, long)]
    pub(crate) baseline: Option<FileOrStdin>,

    /// File to write the updated approved snapshot to.
    #[arg(short, long)]
    pub(crate) output: Option<FileOrStdout>,
}
