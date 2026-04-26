use anyhow::anyhow;
use console::style;

use crate::{
    args::inspect::InspectArgs,
    config::Config,
    helper::{CatalogSession, SnapshotPath, read_snapshot},
};

pub(super) fn dispatch(_config: &Config, args: InspectArgs) -> anyhow::Result<()> {
    let session = CatalogSession::open()?;
    let snapshot = read_snapshot(session.catalog(), args.snapshot, SnapshotPath::Canon)?;

    if args.raw {
        println!("{}", serde_json::to_string_pretty(&snapshot)?);
    } else if let Some(label) = args.label {
        let batch = snapshot
            .into_entries()
            .remove(&label)
            .ok_or_else(|| anyhow!("label not found in snapshot: {label}"))?;
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
    } else {
        let mut entries = snapshot.into_entries().into_iter().collect::<Vec<_>>();
        entries.sort_by(|a, b| a.0.cmp(&b.0));

        for (label, batch) in entries {
            println!(
                "{label}: {}: {}",
                humantime::format_rfc3339_seconds(*batch.created()),
                batch.samples().len(),
            );
        }
    }

    Ok(())
}
