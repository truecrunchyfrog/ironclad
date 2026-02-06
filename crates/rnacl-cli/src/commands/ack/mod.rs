use std::io::{self, Write, stdin};

use anyhow::anyhow;
use rnacl_core::{
    node::id::NodeId,
    sample::{Sample, batch::Batch},
    snapshot::diff::{BatchDiff, SamplePresence},
};

use crate::{
    args::ack::AckArgs,
    helper::{resolve_explicit_or_reused_node_id, resolve_ledger},
    output, ui,
};

pub(super) fn dispatch(args: AckArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let node_ids = args
        .node_id
        .into_iter()
        .map(|node_id| resolve_explicit_or_reused_node_id(&ledger, Some(node_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;
    let dependency_node_ids = args
        .dependency
        .into_iter()
        .map(|node_id| resolve_explicit_or_reused_node_id(&ledger, Some(node_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let audit = ledger.load_pending_snapshot().unwrap_or_default();
    let mut baseline = ledger.load_baseline_snapshot().unwrap_or_default();
    let mut diffs = audit.diff(baseline.clone());

    let node_ids = if !node_ids.is_empty() {
        node_ids
    } else {
        audit
            .entries()
            .keys()
            .map(|node_id| node_id.clone())
            .collect::<Vec<_>>()
    };

    'nodes: for node_id in node_ids {
        let (diff, dep_diffs) = diffs
            .remove(&node_id)
            .ok_or_else(|| anyhow!("node absent in both pending and baseline snapshot"))?;

        let baseline_entry = baseline.entries_mut().entry(node_id.clone()).or_default();

        let interactive = !args.all;

        if dependency_node_ids.is_empty() {
            let should_quit = !ack_batch_diff(
                BatchOrigin::DirtyNode(node_id),
                diff,
                baseline_entry.batch_mut(),
                interactive,
            )?;
            if should_quit {
                break;
            }
        } else {
            for (dep_node_id, dep_diff) in dep_diffs
                .into_iter()
                .filter(|(node_id, _)| dependency_node_ids.contains(node_id))
            {
                let should_quit = ack_batch_diff(
                    BatchOrigin::StaleDependencyNode {
                        dependent: node_id.clone(),
                        dependency: dep_node_id.clone(),
                    },
                    dep_diff,
                    baseline_entry
                        .dependencies_mut()
                        .entry(dep_node_id)
                        .or_default(),
                    interactive,
                )?;
                if should_quit {
                    break 'nodes;
                }
            }
        }
    }

    ledger.save_baseline_snapshot(baseline)?;

    Ok(())
}

enum BatchOrigin {
    DirtyNode(NodeId),
    StaleDependencyNode {
        dependent: NodeId,
        dependency: NodeId,
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
            BatchOrigin::DirtyNode(node_id) => {
                println!("{} is dirty", node_id)
            }
            BatchOrigin::StaleDependencyNode {
                dependent,
                dependency,
            } => println!("{} is stale due to dependency of {}", dependent, dependency),
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
                working_batch.samples_mut().retain(|s| s != &sample)
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
    println!("{}", output::display_sample_diff(&sample_diff));
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
            "s" | "sample" => println!("{}", output::display_sample_diff(&sample_diff)),
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
