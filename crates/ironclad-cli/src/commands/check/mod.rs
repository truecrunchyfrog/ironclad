use crate::{args::check::CheckArgs, context::Context, helper::read_snapshot};

use ironclad_core::catalog::SnapshotFile;

pub(super) fn dispatch(context: &Context, args: CheckArgs) -> anyhow::Result<()> {
    let repository = context.catalog_repository()?;
    let proposal = read_snapshot(&repository, args.proposal, SnapshotFile::Actual)?;
    let baseline = read_snapshot(&repository, args.baseline, SnapshotFile::Canon)?;

    let diff = proposal.diff(&baseline);

    let total = diff.len();

    let equal = diff
        .iter()
        .filter(|(_, batch_diff)| batch_diff.batches_equal())
        .count();

    let unequal = total - equal;

    println!("{} ({unequal})", if unequal == 0 { "ok" } else { "drift" });

    std::process::exit(if unequal == 0 { 0 } else { 1 });
}
