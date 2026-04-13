use std::time::Duration;

use console::style;

use crate::{
    args::review::ReviewArgs,
    batch_origin::BatchOrigin,
    config::Config,
    helper::{
        collect_changed_snapshot_diffs, find_batch_diff, resolve_catalog, set_snapshot_batch,
    },
    output::{self, DisplayPromptOption, PromptOption, format_batch_diff, format_sample_diff},
    ui,
};

enum ReviewState<'a> {
    Overview,
    Batch(&'a BatchOrigin),
    Sample(&'a BatchOrigin, usize),
}

pub(super) fn dispatch(_config: &Config, _args: ReviewArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let audit = catalog.load_candidate_snapshot().unwrap_or_default();
    let baseline = catalog.load_baseline_snapshot().unwrap_or_default();
    let mut working_baseline = baseline.clone();

    let relevant_diffs = collect_changed_snapshot_diffs(audit.diff(&baseline));

    if relevant_diffs.iter().all(|(_, diff)| diff.batches_equal()) {
        println!("nothing to review");
        return Ok(());
    }

    let mut state = ReviewState::Overview;

    loop {
        match state {
            ReviewState::Overview => {
                let working_diff_audit = audit.diff(&working_baseline);
                let working_diff_baseline = working_baseline.diff(&baseline);

                for ((origin, diff), index) in relevant_diffs.iter().zip(1..).collect::<Vec<_>>() {
                    let any_changes = find_batch_diff(origin, &working_diff_baseline)
                        .map_or(false, |d| !d.batches_equal());
                    let resolved_to_baseline = find_batch_diff(origin, &working_diff_audit)
                        .map_or(true, |d| d.batches_equal());
                    println!(
                        "{index} {}{}",
                        if resolved_to_baseline {
                            style("resolved ").green().bold().to_string()
                        } else if any_changes {
                            style("partial  ").yellow().bold().to_string()
                        } else {
                            String::from("         ")
                        },
                        format_batch_diff(origin, diff)
                    );
                }

                println!(
                    "{}/{} batches resolved",
                    relevant_diffs.len() - collect_changed_snapshot_diffs(working_diff_audit).len(),
                    relevant_diffs.len()
                );

                enum OverviewPromptResponse {
                    GoToBatch(usize),
                    QuitWithSave,
                    QuitWithoutSave,
                }

                let result = output::prompt(vec![
                    (
                        PromptOption::Dynamic(|i| {
                            if i.starts_with("g") && i.len() > 1 {
                                if let Ok(index) =
                                    i.chars().skip(1).collect::<String>().parse::<usize>()
                                {
                                    Some(Ok(OverviewPromptResponse::GoToBatch(index)))
                                } else {
                                    Some(Err(
                                        "'g' must be followed by a number of the batch to review.",
                                    ))
                                }
                            } else {
                                None
                            }
                        }),
                        DisplayPromptOption::AlwaysVisible {
                            command: style("gN").blue(),
                            description: style("review batch N").dim(),
                        },
                    ),
                    (
                        PromptOption::Simple("q", || OverviewPromptResponse::QuitWithSave),
                        DisplayPromptOption::AlwaysVisible {
                            command: style("q").green(),
                            description: style("save and quit").dim(),
                        },
                    ),
                    (
                        PromptOption::Simple("Q", || OverviewPromptResponse::QuitWithoutSave),
                        DisplayPromptOption::AlwaysVisible {
                            command: style("Q").red(),
                            description: style("abort changes and quit").dim(),
                        },
                    ),
                ])?;

                match result {
                    OverviewPromptResponse::GoToBatch(index) => {
                        match index.checked_sub(1).and_then(|zero_based_index| {
                            relevant_diffs.iter().nth(zero_based_index)
                        }) {
                            Some((origin, _)) => state = ReviewState::Batch(origin),
                            None => ui::error(format!("no batch at position {index}.")),
                        }
                    }
                    OverviewPromptResponse::QuitWithSave => {
                        catalog.save_baseline_snapshot(working_baseline)?;
                        return Ok(());
                    }
                    OverviewPromptResponse::QuitWithoutSave => return Ok(()),
                };
            }
            ReviewState::Batch(origin) => {
                let diff = relevant_diffs
                    .iter()
                    .find_map(|(origin2, diff)| if origin2 == origin { Some(diff) } else { None })
                    .unwrap();

                println!("{}", format_batch_diff(origin, diff));

                if let Some(before) = diff.before() {
                    println!(
                        "baseline batch is {} old",
                        humantime::format_duration(Duration::from_secs(
                            before.created().elapsed().unwrap().as_secs()
                        ))
                    );
                }

                if let Some(after) = diff.after() {
                    println!(
                        "candidate batch is {} old",
                        humantime::format_duration(Duration::from_secs(
                            after.created().elapsed().unwrap().as_secs()
                        ))
                    );
                }

                let sample_diffs = diff.sample_diffs();

                for ((sample, presence), index) in &sample_diffs.iter().zip(1..).collect::<Vec<_>>()
                {
                    println!("{index} {}", format_sample_diff(sample, &presence));
                }

                enum BatchPromptResponse {
                    AckBatch,
                    GoToSample(usize),
                    GoToOverview,
                }

                let result = output::prompt(vec![
                    (
                        PromptOption::Simple("K", || BatchPromptResponse::AckBatch),
                        DisplayPromptOption::AlwaysVisible {
                            command: style("K").green().bold(),
                            description: style("acknowledge batch").dim(),
                        },
                    ),
                    (
                        PromptOption::Dynamic(|i| {
                            if i.starts_with("g") && i.len() > 1 {
                                if let Ok(index) =
                                    i.chars().skip(1).collect::<String>().parse::<usize>()
                                {
                                    Some(Ok(BatchPromptResponse::GoToSample(index)))
                                } else {
                                    Some(Err(
                                        "'g' must be followed by a number of the sample to review.",
                                    ))
                                }
                            } else {
                                None
                            }
                        }),
                        DisplayPromptOption::AlwaysVisible {
                            command: style("gN").blue(),
                            description: style("review sample N").dim(),
                        },
                    ),
                    (
                        PromptOption::Simple("g", || BatchPromptResponse::GoToOverview),
                        DisplayPromptOption::AlwaysVisible {
                            command: style("g").blue(),
                            description: style("return to overview").dim(),
                        },
                    ),
                ])?;

                match result {
                    BatchPromptResponse::AckBatch => {
                        set_snapshot_batch(origin, &mut working_baseline, diff.after().clone());
                    }
                    BatchPromptResponse::GoToSample(index) => {
                        match index.checked_sub(1).filter(|zero_based_index| {
                            sample_diffs.iter().nth(*zero_based_index).is_some()
                        }) {
                            Some(index) => state = ReviewState::Sample(origin, index),
                            None => ui::error(format!("no sample at position {index}.")),
                        }
                    }
                    BatchPromptResponse::GoToOverview => state = ReviewState::Overview,
                }
            }
            ReviewState::Sample(origin, index) => {
                let diff = relevant_diffs
                    .iter()
                    .find_map(|(origin2, diff)| if origin2 == origin { Some(diff) } else { None })
                    .unwrap();

                let (sample, presence) = diff.sample_diffs().into_iter().nth(index).unwrap();

                println!("{}", format_sample_diff(sample, &presence));

                enum SamplePromptResponse {
                    AckSample,
                    GoToBatch,
                    PipeToCommand(String),
                    PipeToPager,
                    PipeToEditor,
                    QuitWithSave,
                    QuitWithoutSave,
                }

                let result = output::prompt(vec![
                    (
                        PromptOption::Simple("K", || SamplePromptResponse::AckSample),
                        DisplayPromptOption::AlwaysVisible {
                            command: style("K").green().bold(),
                            description: style("acknowledge sample").dim(),
                        },
                    ),
                    (
                        PromptOption::Simple("g", || SamplePromptResponse::GoToBatch),
                        DisplayPromptOption::AlwaysVisible {
                            command: style("g").blue(),
                            description: style("return to batch").dim(),
                        },
                    ),
                    (
                        PromptOption::Simple("v", || SamplePromptResponse::PipeToPager),
                        DisplayPromptOption::Collapsed {
                            command: style("v").yellow(),
                            description: style(
                                format!(
                                    "pipe to $PAGER = {}",
                                    std::env::var("PAGER").unwrap_or(String::from("n/a"))
                                )
                                .as_str(),
                            )
                            .dim(),
                        },
                    ),
                    (
                        PromptOption::Simple("V", || SamplePromptResponse::PipeToEditor),
                        DisplayPromptOption::Collapsed {
                            command: style("V").yellow(),
                            description: style(
                                format!(
                                    "pipe to $EDITOR = {}",
                                    std::env::var("EDITOR").unwrap_or(String::from("n/a"))
                                )
                                .as_str(),
                            )
                            .dim(),
                        },
                    ),
                    (
                        PromptOption::Simple("x", || {
                            SamplePromptResponse::PipeToCommand(String::new())
                        }),
                        DisplayPromptOption::Collapsed {
                            command: style("x").yellow(),
                            description: style("pipe to command").dim(),
                        },
                    ),
                    (
                        PromptOption::Simple("q", || SamplePromptResponse::QuitWithSave),
                        DisplayPromptOption::Collapsed {
                            command: style("q").green(),
                            description: style("save and quit").dim(),
                        },
                    ),
                    (
                        PromptOption::Simple("Q", || SamplePromptResponse::QuitWithoutSave),
                        DisplayPromptOption::Collapsed {
                            command: style("Q").red(),
                            description: style("abort changes and quit").dim(),
                        },
                    ),
                ])?;

                match result {
                    SamplePromptResponse::AckSample => todo!(),
                    SamplePromptResponse::GoToBatch => todo!(),
                    SamplePromptResponse::PipeToCommand(_) => todo!(),
                    SamplePromptResponse::PipeToPager => todo!(),
                    SamplePromptResponse::PipeToEditor => todo!(),
                    SamplePromptResponse::QuitWithSave => todo!(),
                    SamplePromptResponse::QuitWithoutSave => todo!(),
                }
            }
        };
    }
}
