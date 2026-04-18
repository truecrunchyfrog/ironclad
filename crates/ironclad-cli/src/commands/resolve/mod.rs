use std::{
    fs::File,
    io::{BufWriter, Write},
};

use ironclad_core::fact::Fact;

use crate::{args::resolve::ResolveArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: ResolveArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let index = catalog.load_fact_index()?;
    let facts = index
        .into_entries()
        .into_iter()
        .filter(|(label, _)| match &args {
            ResolveArgs { include, .. } if !include.is_empty() => include.contains(&label),
            ResolveArgs { exclude, .. } if !exclude.is_empty() => !exclude.contains(&label),
            _ => true,
        })
        .map(|(label, fact_id)| -> anyhow::Result<(String, Fact)> {
            Ok((
                label,
                catalog.load_fact_for_path(&catalog.fact_file_path(&fact_id))?,
            ))
        })
        .collect::<anyhow::Result<_>>()?;
    let snapshot = catalog.capture_snapshot(facts, !args.no_redact)?;

    let mut dest: Box<dyn Write> = match args.destination {
        Some(file_or_stdout) => Box::new(file_or_stdout.into_writer()?),
        None => Box::new(BufWriter::new(File::create(
            catalog.snapshot_candidate_file_path(),
        )?)),
    };

    dest.write(serde_json::to_vec_pretty(&snapshot)?.as_slice())?;

    Ok(())
}
