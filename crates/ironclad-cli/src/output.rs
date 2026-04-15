use console::{Style, style};
use ironclad_core::{
    fact::id::FactId,
    sample::Sample,
    snapshot::diff::{BatchDiff, SamplePresence},
};

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
