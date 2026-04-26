use anyhow::anyhow;
use ironclad_core::{catalog::SnapshotFile, sample::Sample};

use crate::{args::inspect::InspectArgs, context::Context, helper::read_snapshot};

pub(super) fn dispatch(context: &Context, args: InspectArgs) -> anyhow::Result<()> {
    let repository = context.catalog_repository()?;
    let snapshot = read_snapshot(&repository, args.snapshot, SnapshotFile::Canon)?;

    if args.raw {
        println!("{}", serde_json::to_string_pretty(&snapshot)?);
    } else if let Some(label) = args.label {
        let batch = snapshot
            .into_batch(&label)
            .ok_or_else(|| anyhow!("label not found in snapshot: {label}"))?;
        let samples = batch.into_samples();
        render_detail(&label, &samples, args.trace);
    } else {
        render_summary(snapshot);
    }

    Ok(())
}

fn render_summary(snapshot: ironclad_core::snapshot::Snapshot) {
    for (label, batch) in snapshot.into_sorted_entries() {
        println!(
            "{}  {}  {}",
            label,
            batch.samples().len(),
            humantime::format_rfc3339_seconds(*batch.created()),
        );
    }
}

fn render_detail(label: &str, samples: &[Sample], show_trace: bool) {
    println!("{label}");
    println!();

    for (index, sample) in samples.iter().enumerate() {
        println!("{}.", index + 1);

        if show_trace {
            for trace in sample.traces() {
                let mut entries = trace.entries().iter().collect::<Vec<_>>();
                entries.sort_by(|a, b| a.0.cmp(b.0));
                println!(
                    "trace: {}",
                    entries
                        .into_iter()
                        .map(|(k, v)| format!("{k}={v}"))
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
        }

        print_sample(sample);
        println!();
    }
}

fn print_sample(sample: &Sample) {
    if sample.content().contains('\n') {
        println!("content:");
        println!("<<<");
        println!("{}", sample.content());
        println!(">>>");
    } else {
        println!("content: {:?}", sample.content());
    }
}
