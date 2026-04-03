use clap::Args;

/// Capture snapshot of current state and compare against baseline.
#[derive(Args)]
pub(crate) struct AuditArgs {
    /// ID of cell to show.
    pub(crate) cell_id: Vec<String>,

    /// Show confliced samples.
    #[arg(short = 'd', long)]
    pub(crate) expand_diff: bool,

    /// Show stale dependencies.
    #[arg(short = 's', long)]
    pub(crate) expand_stale: bool,

    /// Use cache instead of creating a new audit.
    #[arg(short, long)]
    pub(crate) cache: bool,

    /// Capture a fresh snapshot, without including cache from audit snapshot.
    #[arg(short, long, conflicts_with = "cache")]
    pub(crate) fresh: bool,

    /// Don't set the audit as pending state.
    #[arg(long)]
    pub(crate) dry_run: bool,
}
