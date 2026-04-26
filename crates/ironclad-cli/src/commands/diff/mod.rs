use anyhow::anyhow;
use console::style;
use ironclad_core::{
    catalog::SnapshotFile,
    sample::Sample,
    snapshot::diff::{BatchDiff, BatchStatus, SampleChangeKind},
};

use crate::{args::diff::DiffArgs, context::Context, helper::read_snapshot};

pub(super) fn dispatch(context: &Context, args: DiffArgs) -> anyhow::Result<()> {
    let repository = context.catalog_repository()?;
    let proposal = read_snapshot(&repository, args.proposal, SnapshotFile::Actual)?;
    let baseline = read_snapshot(&repository, args.baseline, SnapshotFile::Canon)?;

    let mut diff = proposal.diff(&baseline);

    if args.raw {
        println!("{}", serde_json::to_string_pretty(&diff)?);
    } else if let Some(label) = args.label {
        let batch_diff = diff
            .remove(&label)
            .ok_or_else(|| anyhow!("label not found in compared snapshots: {label}"))?;
        render_detail(&label, &batch_diff, args.trace);
    } else {
        render_summary(&proposal, &baseline);
    }

    Ok(())
}

fn render_summary(
    proposal: &ironclad_core::snapshot::Snapshot,
    baseline: &ironclad_core::snapshot::Snapshot,
) {
    for (label, batch_diff) in proposal.sorted_diff(baseline) {
        if batch_diff.status() == BatchStatus::Unchanged {
            continue;
        }

        let counts = batch_diff.change_counts();
        println!(
            "{}  -{} +{}  {}",
            format_status(batch_diff.status()),
            counts.removed,
            counts.added,
            label
        );
    }
}

fn render_detail(label: &str, batch_diff: &BatchDiff, show_trace: bool) {
    println!("{label}");
    println!();

    for (index, change) in batch_diff.sample_changes().into_iter().enumerate() {
        println!("{}. {}", index + 1, format_change_kind(change.kind()));

        if show_trace {
            if let Some(before) = change.before() {
                print_trace("before.trace", before);
            }
            if let Some(after) = change.after() {
                let duplicate = change
                    .before()
                    .is_some_and(|before| before.traces() == after.traces());
                if !duplicate {
                    print_trace("after.trace", after);
                }
            }
        }

        if let Some(before) = change.before() {
            print_sample_side("before", before);
        }
        if let Some(after) = change.after() {
            let duplicate = change
                .before()
                .is_some_and(|before| before.content() == after.content());
            if !duplicate || change.kind() != SampleChangeKind::Unchanged {
                print_sample_side("after", after);
            }
        }

        println!();
    }
}

fn print_trace(prefix: &str, sample: &Sample) {
    for trace in sample.traces() {
        let mut entries = trace.entries().iter().collect::<Vec<_>>();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        println!(
            "{}: {}",
            prefix,
            entries
                .into_iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}

fn print_sample_side(prefix: &str, sample: &Sample) {
    if sample.content().contains('\n') {
        println!("{prefix}:");
        println!("<<<");
        println!("{}", sample.content());
        println!(">>>");
    } else {
        println!("{prefix}: {:?}", sample.content());
    }
}

fn format_status(status: BatchStatus) -> console::StyledObject<&'static str> {
    match status {
        BatchStatus::New => style("new").green(),
        BatchStatus::Removed => style("removed").red(),
        BatchStatus::Changed => style("changed").yellow(),
        BatchStatus::Unchanged => style("unchanged").dim(),
    }
}

fn format_change_kind(kind: SampleChangeKind) -> console::StyledObject<&'static str> {
    match kind {
        SampleChangeKind::Removed => style("removed").red(),
        SampleChangeKind::Added => style("added").green(),
        SampleChangeKind::Unchanged => style("unchanged").dim(),
    }
}
