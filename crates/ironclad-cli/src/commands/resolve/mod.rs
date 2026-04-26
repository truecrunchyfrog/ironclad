use std::{
    fs::File,
    io::{BufWriter, Write},
};

use anyhow::anyhow;
use ironclad_core::{
    catalog::SnapshotProgressEvent,
    fact::{LabeledFact, RecipeProgressEvent},
};

use crate::{args::resolve::ResolveArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: ResolveArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let index = catalog.load_fact_index()?;

    let no_redact = args.no_redact;

    let facts = match args {
        ResolveArgs { include, .. } if !include.is_empty() => include
            .into_iter()
            .map(|label| {
                let fact = catalog.load_fact_for_path(
                    &catalog.fact_file_path(
                        &index
                            .id_for_label(&label)
                            .ok_or_else(|| anyhow!("absent from index: {label}"))?,
                    ),
                )?;
                Ok(LabeledFact { label, fact })
            })
            .collect::<anyhow::Result<Vec<_>>>()?,
        ResolveArgs { exclude, .. } => {
            let mut entries = index.into_entries();
            for label in exclude {
                entries
                    .remove(&label)
                    .ok_or_else(|| anyhow!("absent from index: {label}"))?;
            }
            entries
                .into_iter()
                .map(|(label, fact_id)| {
                    Ok(LabeledFact {
                        label,
                        fact: catalog.load_fact_for_path(&catalog.fact_file_path(&fact_id))?,
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?
        }
    };

    let total = facts.len();

    eprint!("...");

    let result_snapshot = catalog.capture_snapshot(facts, !no_redact, |update| match update {
        SnapshotProgressEvent::FactStep {
            index,
            fact,
            inner:
                RecipeProgressEvent::StepStarted {
                    index: step_index,
                    step,
                    ..
                },
            ..
        } => {
            eprint!(
                "\r\x1b[2K{}/{total}: {}: {}/{}: {}",
                index + 1,
                fact.label,
                step_index + 1,
                fact.steps().len(),
                step.operation_id()
            );
            let _ = std::io::stderr().flush();
        }
        _ => (),
    });

    eprint!("\r\x1b[2K");

    let snapshot = match result_snapshot {
        Ok(snapshot) => snapshot,
        Err(err) => return Err(err.into()),
    };

    let mut dest: Box<dyn Write> = match args.output {
        Some(file_or_stdout) => Box::new(file_or_stdout.into_writer()?),
        None => Box::new(BufWriter::new(File::create(
            catalog.snapshot_actual_file_path(),
        )?)),
    };

    dest.write(serde_json::to_vec_pretty(&snapshot)?.as_slice())?;

    Ok(())
}
