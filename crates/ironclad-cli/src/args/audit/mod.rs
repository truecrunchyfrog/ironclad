use clap::Args;

#[derive(Args)]
pub(crate) struct AuditArgs {
    /// ID of node to limit view to.
    pub(crate) node_id: Vec<String>,

    /// Show confliced samples.
    #[arg(short = 'd', long)]
    pub(crate) expand_diff: bool,

    /// Show stale dependencies.
    #[arg(short = 's', long)]
    pub(crate) expand_stale: bool,

    /// Use existing pending snapshot instead of capturing a new snapshot.
    #[arg(short, long)]
    pub(crate) cache: bool,

    /// Capture a fresh snapshot, ignoring any cached batches.
    #[arg(short, long, conflicts_with = "cache")]
    pub(crate) new: bool,

    /// Don't set the audit as pending state.
    #[arg(long)]
    pub(crate) dry_run: bool,
}
