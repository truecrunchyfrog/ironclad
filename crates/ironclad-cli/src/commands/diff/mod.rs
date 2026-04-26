use std::{
    fs::File,
    io::{BufReader, Read},
};

use console::style;
use ironclad_core::snapshot::{
    Snapshot,
    diff::{BatchDiff, SamplePresence},
};

use crate::{args::diff::DiffArgs, config::Config, helper::resolve_catalog, output};

pub(super) fn dispatch(_config: &Config, args: DiffArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let proposal = serde_json::from_reader::<Box<dyn Read>, Snapshot>(match args.proposal {
        Some(file_or_stdin) => Box::new(file_or_stdin.into_reader()?),
        None => Box::new(BufReader::new(File::open(
            catalog.snapshot_actual_file_path(),
        )?)),
    })?;

    let baseline = serde_json::from_reader::<Box<dyn Read>, Snapshot>(match args.baseline {
        Some(file_or_stdin) => Box::new(file_or_stdin.into_reader()?),
        None => Box::new(BufReader::new(File::open(
            catalog.snapshot_canon_file_path(),
        )?)),
    })?;

    let mut diff = proposal.diff(&baseline);

    if args.raw {
        println!("{}", serde_json::to_string_pretty(&diff)?);
    } else if let Some(label) = args.label {
        if let Some(batch_diff) = diff.remove(&label) {
            for ((sample, presence), i) in batch_diff
                .sample_diffs()
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
                    println!("{}", sample.content());
                } else {
                    println!(
                        "{i:2}: {}\n{}",
                        match presence {
                            SamplePresence::OnlyBefore => style("-").red(),
                            SamplePresence::OnlyAfter => style("+").green(),
                            SamplePresence::Both => style("=").blue(),
                        },
                        {
                            let s = style(sample.content());

                            match presence {
                                SamplePresence::OnlyBefore => s.black().on_red(),
                                SamplePresence::OnlyAfter => s.black().on_green(),
                                SamplePresence::Both => s.black().on_blue(),
                            }
                        }
                    );
                }
            }
        }
    } else {
        for (label, batch_diff) in &diff {
            if !batch_diff.batches_equal() {
                println!("{}", format_batch_diff(label, batch_diff));
            }
        }
    }

    Ok(())
}

fn format_batch_diff(label: &str, diff: &BatchDiff) -> String {
    let status = match (diff.before(), diff.after()) {
        (None, Some(_)) => style("new").green(),
        (Some(_), None) => style("old").red(),
        (Some(_), Some(_)) if diff.batches_equal() => style("ok!"),
        (Some(_), Some(_)) => style("dft").yellow(),
        _ => unreachable!(),
    };

    let dirtiness = output::format_dirtiness(
        diff.sample_diffs()
            .into_iter()
            .map(|(_, presence)| presence)
            .collect::<Vec<_>>()
            .as_slice(),
    );

    format!("{status} {dirtiness} {label}")
}
