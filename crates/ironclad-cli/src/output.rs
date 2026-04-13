use std::{
    fmt::Display,
    io::{Write, stdin, stdout},
};

use console::{Style, style};
use ironclad_core::{
    fact::id::FactId,
    sample::Sample,
    snapshot::diff::{BatchDiff, SamplePresence},
};

use crate::ui;

pub(crate) fn format_sample_diff(sample: &Sample, presence: &SamplePresence) -> String {
    let style = match presence {
        SamplePresence::OnlyBefore => Style::new().red(),
        SamplePresence::OnlyAfter => Style::new().green(),
        SamplePresence::Both => Style::new(),
    };
    format!(
        "{}\n{}",
        style.apply_to(match presence {
            SamplePresence::OnlyBefore => "-",
            SamplePresence::OnlyAfter => "+",
            SamplePresence::Both => "=",
        }),
        style.apply_to(
            sample
                .content()
                .lines()
                .map(|line| format!("  {line}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    )
}

pub(crate) fn format_dirtiness(presences: &[SamplePresence]) -> String {
    let removed = presences
        .iter()
        .filter(|p| matches!(p, SamplePresence::OnlyBefore))
        .count();
    let added = presences
        .iter()
        .filter(|p| matches!(p, SamplePresence::OnlyAfter))
        .count();

    let display_removed = format!("-{removed}");
    let display_added = format!("+{added}");

    format!(
        "{} {}",
        if removed != 0 {
            style(display_removed).red()
        } else {
            style(display_removed).dim()
        },
        if added != 0 {
            style(display_added).green()
        } else {
            style(display_added).dim()
        },
    )
}

pub(crate) fn format_batch_diff(fact_id: &FactId, diff: &BatchDiff) -> String {
    let status = match (diff.before(), diff.after()) {
        (None, Some(_)) => style("add").green(),
        (Some(_), None) => style("rem").red(),
        (Some(_), Some(_)) if diff.batches_equal() => style("ok!").black().on_green(),
        (Some(_), Some(_)) => style("mut").yellow(),
        _ => unreachable!(),
    };

    let dirtiness = format_dirtiness(
        diff.sample_diffs()
            .into_iter()
            .map(|(_, presence)| presence)
            .collect::<Vec<_>>()
            .as_slice(),
    );

    format!("{status} {dirtiness} {fact_id}")
}

pub(crate) enum PromptOption<'a, T> {
    Simple(&'a str, fn() -> T),
    Dynamic(fn(&str) -> Option<Result<T, &str>>),
}

pub(crate) enum DisplayPromptOption<F: Display> {
    AlwaysVisible { command: F, description: F },
    Collapsed { command: F, description: F },
}

pub(crate) fn prompt<'a, T, F>(
    options: Vec<(PromptOption<'a, T>, DisplayPromptOption<F>)>,
) -> anyhow::Result<T>
where
    F: Display + 'a,
{
    let expandable = options
        .iter()
        .any(|(_, display_option)| matches!(display_option, DisplayPromptOption::Collapsed { .. }));
    let mut expanded = false;

    loop {
        println!("");
        for (_, display_option) in &options {
            match display_option {
                DisplayPromptOption::AlwaysVisible {
                    command,
                    description,
                } => println!("   {command}\t{description}"),
                DisplayPromptOption::Collapsed {
                    command,
                    description,
                } if expanded => println!("   {command}\t{description}"),
                _ => (),
            }
        }

        if expandable && !expanded {
            println!(
                "   {}\t{}",
                style("?").dim(),
                style("show all commands").dim()
            );
        }

        stdout().flush()?;

        let mut choice = String::new();
        stdin().read_line(&mut choice)?;

        let choice = choice.trim();

        let result = options
            .iter()
            .find_map(|(prompt_option, _)| match prompt_option {
                PromptOption::Simple(command, result) if &choice == command => Some(Ok(result())),
                PromptOption::Dynamic(determiner) => determiner(choice),
                _ => None,
            });

        match result {
            Some(Ok(result)) => return Ok(result),
            Some(Err(err)) => ui::error(err),
            None if expandable && (choice == "?" || expanded && choice == "") => {
                expanded = !expanded
            }
            None if choice == "" => ui::error(format!("please enter a command.")),
            None => ui::error(format!("no such command '{choice}'.")),
        };
    }
}
