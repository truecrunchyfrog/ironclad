use clap::Args;
use clap_stdin::FileOrStdin;

/// Check whether two snapshots are identical.
#[derive(Args)]
#[command(long_about = "Ensure two snapshots are identical.\n\n\
`check` is the non-interactive summary form of snapshot comparison. It reports \
whether drift exists and exits with status `0` when the snapshots match or `1` \
when they differ.\n\n\
By default it compares the resolution snapshot in `actual.json` with the approved \
snapshot in `canon.json`. Both inputs can be overridden.")]
pub(crate) struct CheckArgs {
    /// Resolution snapshot to compare from.
    #[arg(short, long)]
    pub(crate) proposal: Option<FileOrStdin>,

    /// Approved snapshot to compare against.
    #[arg(short, long)]
    pub(crate) baseline: Option<FileOrStdin>,
}
