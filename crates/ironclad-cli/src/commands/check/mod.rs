use crate::{
    args::check::CheckArgs,
    config::Config,
    helper::{CatalogSession, SnapshotPath, read_snapshot},
};

pub(super) fn dispatch(_config: &Config, args: CheckArgs) -> anyhow::Result<()> {
    let session = CatalogSession::open()?;
    let proposal = read_snapshot(session.catalog(), args.proposal, SnapshotPath::Actual)?;
    let baseline = read_snapshot(session.catalog(), args.baseline, SnapshotPath::Canon)?;

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
