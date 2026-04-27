use clap::Args;
use clap_stdin::FileOrStdout;

/// Resolve facts into a snapshot.
#[derive(Args)]
#[command(long_about = "Capture a snapshot of currently evaluated state.\n\n\
`resolve` evaluates fact pipelines and writes the resulting resolved snapshot. \
Without selectors, it resolves every indexed fact.\n\n\
Use positional labels to resolve only specific facts, `--exclude` to skip \
selected labels, `--output` to write elsewhere, and `--no-redact` to keep \
secret facts unredacted in the result.")]
pub(crate) struct ResolveArgs {
    /// Labels to resolve.
    pub(crate) include: Vec<String>,

    /// Labels to exclude from a full resolve.
    #[arg(short = 'x', long, conflicts_with = "include")]
    pub(crate) exclude: Vec<String>,

    /// File to write the resolved snapshot to.
    #[arg(short, long)]
    pub(crate) output: Option<FileOrStdout>,

    /// Keep secret facts unredacted in the snapshot output.
    #[arg(long)]
    pub(crate) no_redact: bool,
}
