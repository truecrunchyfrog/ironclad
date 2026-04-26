use std::{
    fs::File,
    io::{BufReader, Read},
};

use ironclad_core::snapshot::Snapshot;

use crate::{args::check::CheckArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: CheckArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let proposal = serde_json::from_reader::<Box<dyn Read>, Snapshot>(match args.proposal {
        Some(file_or_stdin) => Box::new(file_or_stdin.into_reader()?),
        None => Box::new(BufReader::new(File::open(
            catalog.snapshot_actual_file_path(),
        )?)),
    })?;

    let baseline = serde_json::from_reader::<Box<dyn Read>, Snapshot>(match args.baseline {
        Some(file_or_stdin) => Box::new(file_or_stdin.into_reader()?),
        None => Box::new(BufReader::new(File::open(
            catalog.snapshot_canon_file_path(),
        )?)),
    })?;

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
