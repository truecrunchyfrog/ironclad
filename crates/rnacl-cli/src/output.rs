use console::Style;
use rnacl_core::{sample::Sample, snapshot::diff::SamplePresence};

pub(crate) fn display_sample_diff(sample_diff: &(Sample, SamplePresence)) -> String {
    let style = match sample_diff.1 {
        SamplePresence::OnlyBefore => Style::new().red(),
        SamplePresence::OnlyAfter => Style::new().green(),
        SamplePresence::Both => Style::new(),
    };
    format!(
        "{}\n{}",
        style.apply_to(match sample_diff.1 {
            SamplePresence::OnlyBefore => "-",
            SamplePresence::OnlyAfter => "+",
            SamplePresence::Both => "=",
        }),
        style.apply_to(
            sample_diff
                .0
                .content()
                .lines()
                .map(|line| format!("  {}", line))
                .collect::<Vec<_>>()
                .join("\n")
        )
    )
}

pub(crate) fn display_dirtiness(sample_diff_presences: Vec<&SamplePresence>) -> String {
    format!(
        "-{} +{}",
        sample_diff_presences
            .iter()
            .filter(|p| matches!(p, SamplePresence::OnlyBefore))
            .count(),
        sample_diff_presences
            .iter()
            .filter(|p| matches!(p, SamplePresence::OnlyAfter))
            .count(),
    )
}
