use clap::Args;

/// Capture snapshot of current state and compare against baseline.
#[derive(Args)]
pub(crate) struct ResolveArgs {
    /// ID of fact to show.
    pub(crate) fact_id: Vec<String>,

    /// Show confliced samples.
    #[arg(short = 'd', long)]
    pub(crate) expand_diff: bool,

    /// Use cache instead of creating a new audit.
    #[arg(short, long)]
    pub(crate) cache: bool,

    /// Capture a fresh snapshot, without including cache from audit snapshot.
    #[arg(short, long, conflicts_with = "cache")]
    pub(crate) fresh: bool,

    /// Don't set the audit as candidate state.
    #[arg(long)]
    pub(crate) dry_run: bool,
}
