use std::{
    fs::File,
    io::{BufReader, Read},
};

use console::style;
use ironclad_core::{fact::id::FactId, snapshot::Snapshot};

use crate::{args::inspect::InspectArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: InspectArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let snapshot = serde_json::from_reader::<Box<dyn Read>, Snapshot>(match args.snapshot {
        Some(file_or_stdin) => Box::new(file_or_stdin.into_reader()?),
        None => Box::new(BufReader::new(File::open(
            catalog.snapshot_baseline_path(),
        )?)),
    })?;

    if args.raw {
        println!("{}", serde_json::to_string_pretty(&snapshot)?);
    } else if let Some(fact_id) = args.fact_id {
        if let Some(batch) = snapshot.into_entries().remove(&FactId::from(fact_id)) {
            for (sample, i) in batch
                .into_samples()
                .into_iter()
                .zip(1..)
                .filter(|(_, i)| args.index.is_none_or(|only_show| only_show == *i))
            {
                let exclusive = args.index.is_some();

                if args.trace {
                    for trace in sample.traces() {
                        println!(
                            "trace: {}",
                            trace
                                .entries()
                                .iter()
                                .map(|(k, v)| format!("{k}={v}"))
                                .collect::<Vec<_>>()
                                .join(" ")
                        );
                    }
                }

                if exclusive {
                    println!("{}", sample.into_content());
                } else {
                    println!(
                        "{i:2}: {}",
                        if i % 2 == 0 {
                            style(sample.into_content()).black().on_white()
                        } else {
                            style(sample.into_content()).black().on_yellow()
                        }
                    );
                }
            }
        }
    } else {
        for (fact_id, batch) in snapshot.into_entries() {
            println!(
                "{fact_id}: {}: {}",
                humantime::format_rfc3339_seconds(*batch.created()),
                batch.samples().len(),
            );
        }
    }

    Ok(())
}
