use std::{
    collections::HashSet,
    io::{self, Write, stdin},
};

use anyhow::anyhow;
use ironclad_core::{
    cell::id::CellId,
    sample::{Sample, batch::Batch},
    snapshot::diff::{BatchDiff, SamplePresence},
};

use crate::{
    args::review::ReviewArgs,
    config::Config,
    helper::{resolve_cluster, resolve_explicit_or_reused_cell_id},
    output, ui,
};

pub(super) fn dispatch(_config: &Config, args: ReviewArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;
    let cell_ids = args
        .cell_id
        .into_iter()
        .map(|cell_id| resolve_explicit_or_reused_cell_id(&cluster, Some(cell_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;
    let dependency_cell_ids = args
        .dependency
        .into_iter()
        .map(|cell_id| resolve_explicit_or_reused_cell_id(&cluster, Some(cell_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let audit = cluster.load_pending_snapshot().unwrap_or_default();
    let mut baseline = cluster.load_baseline_snapshot().unwrap_or_default();
    let mut diffs = audit.diff(baseline.clone());

    let cell_ids = if cell_ids.is_empty() {
        audit
            .entries()
            .keys()
            .chain(baseline.entries().keys())
            .cloned()
            .collect::<HashSet<_>>()
    } else {
        cell_ids.into_iter().collect()
    };

    'cells: for cell_id in cell_ids {
        let (diff, dep_diffs) = diffs
            .remove(&cell_id)
            .ok_or_else(|| anyhow!("cell absent in both pending and baseline snapshot"))?;

        let baseline_entry = baseline.entries_mut().entry(cell_id.clone()).or_default();

        let interactive = !args.all;

        if dependency_cell_ids.is_empty() {
            let should_quit = !ack_batch_diff(
                BatchOrigin::DirtyCell(cell_id),
                diff,
                baseline_entry.batch_mut(),
                interactive,
            )?;
            if should_quit {
                break;
            }
        } else {
            for (dep_cell_id, dep_diff) in dep_diffs
                .into_iter()
                .filter(|(cell_id, _)| dependency_cell_ids.contains(cell_id))
            {
                let should_quit = ack_batch_diff(
                    BatchOrigin::StaleDependencyCell {
                        dependent: cell_id.clone(),
                        dependency: dep_cell_id.clone(),
                    },
                    dep_diff,
                    baseline_entry
                        .dependencies_mut()
                        .entry(dep_cell_id)
                        .or_default(),
                    interactive,
                )?;
                if should_quit {
                    break 'cells;
                }
            }
        }
    }

    cluster.save_baseline_snapshot(baseline)?;

    Ok(())
}

enum BatchOrigin {
    DirtyCell(CellId),
    StaleDependencyCell {
        dependent: CellId,
        dependency: CellId,
    },
}

fn ack_batch_diff(
    origin: BatchOrigin,
    batch_diff: BatchDiff,
    working_batch: &mut Batch,
    interactive: bool,
) -> anyhow::Result<bool> {
    let sample_diffs = batch_diff
        .sample_diffs()
        .into_iter()
        .filter(|(_, presence)| presence != &SamplePresence::Both)
        .collect::<Vec<_>>();

    if interactive && !sample_diffs.is_empty() {
        match origin {
            BatchOrigin::DirtyCell(cell_id) => {
                println!("{cell_id} is dirty");
            }
            BatchOrigin::StaleDependencyCell {
                dependent,
                dependency,
            } => println!("{dependent} is stale due to dependency of {dependency}"),
        }
    }

    for sample_diff in sample_diffs {
        if interactive {
            match prompt_ack_sample_diff(&sample_diff)? {
                SamplePromptResponse::AckSample => (),
                SamplePromptResponse::SkipSample => continue,
                SamplePromptResponse::SkipBatch => break,
                SamplePromptResponse::SkipAll => return Ok(false),
            }
        }

        match sample_diff {
            (sample, SamplePresence::OnlyBefore) => {
                working_batch.samples_mut().retain(|s| s != &sample);
            }
            (sample, SamplePresence::OnlyAfter) => working_batch.samples_mut().push(sample),
            _ => (),
        }
    }

    Ok(true)
}

enum SamplePromptResponse {
    AckSample,
    SkipSample,
    SkipBatch,
    SkipAll,
}

fn prompt_ack_sample_diff(
    sample_diff: &(Sample, SamplePresence),
) -> anyhow::Result<SamplePromptResponse> {
    println!("{}", output::display_sample_diff(sample_diff));
    loop {
        print!("y/n/N/q/s/t/? = ");
        io::stdout().flush()?;

        let mut choice = String::new();
        stdin().read_line(&mut choice)?;

        match choice.trim() {
            "y" | "yes" => return Ok(SamplePromptResponse::AckSample),
            "n" | "no" => return Ok(SamplePromptResponse::SkipSample),
            "N" | "NO" => return Ok(SamplePromptResponse::SkipBatch),
            "q" | "quit" => return Ok(SamplePromptResponse::SkipAll),
            "s" | "sample" => println!("{}", output::display_sample_diff(sample_diff)),
            "t" | "trace" => println!("{}", serde_json::to_string_pretty(sample_diff.0.traces())?),
            "?" | "h" | "help" => println!(
                "\
yes:    ack the sample
no:     skip the sample
NO:     skip the rest of the samples in the batch (keeps previous acks)
quit:   skip the rest of the samples and quit (keeps previous acks)
sample: show the sample
trace:  show the sample's trace to determine its origin
help:   show this

each command has its first letter as an alias"
            ),
            "" => (),
            input => ui::error(format!(
                "no such command '{input}'. try 'help' for an explanation."
            )),
        }
    }
}
