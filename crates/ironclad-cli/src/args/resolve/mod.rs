use clap::Args;
use clap_stdin::FileOrStdout;

/// Resolve facts into a snapshot.
///
/// `resolve` evaluates fact pipelines and writes the resulting resolution snapshot.
/// Without selectors, it resolves every indexed fact.
///
/// Use positional labels to resolve only specific facts, `--exclude` to skip
/// selected labels, `--output` to write elsewhere, and `--no-redact` to keep
/// secret facts unredacted in the result.
#[derive(Args)]
pub(crate) struct ResolveArgs {
    /// Labels to resolve.
    pub(crate) include: Vec<String>,

    /// Labels to exclude from a full resolve.
    #[arg(short = 'x', long, conflicts_with = "include")]
    pub(crate) exclude: Vec<String>,

    /// File to write the resolution snapshot to.
    #[arg(short, long)]
    pub(crate) output: Option<FileOrStdout>,

    /// Keep secret facts unredacted in the snapshot output.
    #[arg(long)]
    pub(crate) no_redact: bool,
}
