use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

use ironclad_core::{catalog::Catalog, sample::batch::Batch, snapshot::Snapshot};

use crate::{args::apply::ApplyArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: ApplyArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let index = catalog.load_fact_index()?;
    let include_fact_ids = args
        .label
        .into_iter()
        .map(|label| Catalog::fact_id_for_label(&index, &label))
        .collect::<Result<Vec<_>, _>>()?;

    let promotion = serde_json::from_reader::<Box<dyn Read>, Snapshot>(match args.promotion {
        Some(file_or_stdin) => Box::new(file_or_stdin.into_reader()?),
        None => Box::new(BufReader::new(File::open(
            catalog.snapshot_candidate_file_path(),
        )?)),
    })?;

    let baseline = serde_json::from_reader::<Box<dyn Read>, Snapshot>(match args.baseline {
        Some(file_or_stdin) => Box::new(file_or_stdin.into_reader()?),
        None => Box::new(BufReader::new(File::open(
            catalog.snapshot_baseline_file_path(),
        )?)),
    })?;

    let mut dest: Box<dyn Write> = match args.destination {
        Some(file_or_stdout) => Box::new(file_or_stdout.into_writer()?),
        None => Box::new(BufWriter::new(File::create(
            catalog.snapshot_baseline_file_path(),
        )?)),
    };

    let accepted_promotion = promotion
        .into_entries()
        .into_iter()
        .filter(|(fact_id, _)| args.all || include_fact_ids.contains(fact_id));

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
