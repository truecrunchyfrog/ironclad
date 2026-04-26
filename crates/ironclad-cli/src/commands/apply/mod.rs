use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

use anyhow::anyhow;
use ironclad_core::{sample::batch::Batch, snapshot::Snapshot};

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

    let accepted_promotion = match args {
        ApplyArgs { all: true, .. } => promotion.into_entries(),
        ApplyArgs {
            all: false,
            label: labels,
            ..
        } => {
            let mut entries = promotion.into_entries();
            labels
                .into_iter()
                .map(|label| {
                    let entry = entries
                        .remove(&label)
                        .ok_or_else(|| anyhow!("absent from proposal: {label}"))?;
                    Ok((label, entry))
                })
                .collect::<anyhow::Result<Vec<_>>>()?
                .into_iter()
                .collect()
        }
    };

    let promoted_baseline = Snapshot::new(
        baseline
            .into_entries()
            .into_iter()
            .chain(accepted_promotion)
            .collect::<HashMap<String, Batch>>(),
    );

    dest.write(serde_json::to_vec_pretty(&promoted_baseline)?.as_slice())?;

    Ok(())
}
