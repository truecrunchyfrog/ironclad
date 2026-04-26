use anyhow::anyhow;
use console::style;

use crate::{
    args::inspect::InspectArgs,
    context::Context,
    helper::{SnapshotPath, read_snapshot},
};

pub(super) fn dispatch(context: &Context, args: InspectArgs) -> anyhow::Result<()> {
    let catalog = context.catalog()?;
    let snapshot = read_snapshot(&catalog, args.snapshot, SnapshotPath::Canon)?;

    if args.raw {
        println!("{}", serde_json::to_string_pretty(&snapshot)?);
    } else if let Some(label) = args.label {
        let batch = snapshot
            .into_batch(&label)
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
        for (label, batch) in snapshot.into_sorted_entries() {
            println!(
                "{label}: {}: {}",
                humantime::format_rfc3339_seconds(*batch.created()),
                batch.samples().len(),
            );
        }
    }

    Ok(())
}
