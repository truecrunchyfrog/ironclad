use std::{
    fs::File,
    io::{BufWriter, Write},
};

use ironclad_core::{
    catalog::SnapshotProgressEvent,
    fact::{LabeledFact, RecipeProgressEvent},
};

use crate::{args::resolve::ResolveArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: ResolveArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let index = catalog.load_fact_index()?;
    let facts = index
        .into_entries()
        .into_iter()
        .filter(|(label, _)| match &args {
            ResolveArgs { include, .. } if !include.is_empty() => include.contains(label),
            ResolveArgs { exclude, .. } if !exclude.is_empty() => !exclude.contains(label),
            _ => true,
        })
        .map(|(label, fact_id)| -> anyhow::Result<_> {
            Ok(LabeledFact {
                label,
                fact: catalog.load_fact_for_path(&catalog.fact_file_path(&fact_id))?,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let total = facts.len();

    eprint!("...");

    let result_snapshot = catalog.capture_snapshot(facts, !args.no_redact, |update| match update {
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
            catalog.snapshot_candidate_file_path(),
        )?)),
    };

    dest.write(serde_json::to_vec_pretty(&snapshot)?.as_slice())?;

    Ok(())
}
