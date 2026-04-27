use clap::{ArgGroup, Args};
use clap_stdin::{FileOrStdin, FileOrStdout};

/// Apply changes into the approved snapshot.
///
/// `apply` promotes facts from the resolution snapshot into the approved snapshot.
/// You can promote selected labels or replace the whole approved snapshot with
/// `--all`.
///
/// By default the resolution snapshot comes from `actual.json` and the approved
/// snapshot comes from `canon.json`. Both can be overridden, and the result can be
/// written to another file instead of replacing `canon.json`.
#[derive(Args)]
#[command(group(
    ArgGroup::new("selection")
        .args(["label", "all"])
        .required(true)
))]
pub(crate) struct ApplyArgs {
    /// Fact labels to promote.
    pub(crate) label: Vec<String>,

    /// Replace the whole approved snapshot.
    #[arg(short, long)]
    pub(crate) all: bool,

    /// Resolution snapshot to promote from.
    #[arg(short, long)]
    pub(crate) promotion: Option<FileOrStdin>,

    /// Approved snapshot to update.
    #[arg(short, long)]
    pub(crate) baseline: Option<FileOrStdin>,

    /// File to write the updated approved snapshot to.
    #[arg(short, long)]
    pub(crate) output: Option<FileOrStdout>,
}
