use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

use anyhow::bail;
use ironclad_core::snapshot::Snapshot;

use crate::{args::apply::ApplyArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: ApplyArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let promotion = serde_json::from_reader::<Box<dyn Read>, Snapshot>(match args.promotion {
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

    let mut dest: Box<dyn Write> = match args.output {
        Some(file_or_stdout) => Box::new(file_or_stdout.into_writer()?),
        None => Box::new(BufWriter::new(File::create(
            catalog.snapshot_canon_file_path(),
        )?)),
    };

    let promoted_baseline = match args {
        ApplyArgs { all: true, .. } => promotion,
        ApplyArgs {
            all: false,
            label: labels,
            ..
        } => {
            let mut baseline_entries = baseline.into_entries();
            let mut promotion_entries = promotion.into_entries();

            for label in labels {
                if baseline_entries.remove(&label).is_none()
                    && !promotion_entries.contains_key(&label)
                {
                    bail!("absent from proposal and baseline: {label}");
                }

                if let Some(entry) = promotion_entries.remove(&label) {
                    baseline_entries.insert(label, entry);
                }
            }

            Snapshot::new(baseline_entries)
        }
    };

    dest.write(serde_json::to_vec_pretty(&promoted_baseline)?.as_slice())?;

    Ok(())
}
