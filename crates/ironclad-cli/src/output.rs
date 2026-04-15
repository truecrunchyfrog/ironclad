use console::style;
use ironclad_core::snapshot::diff::SamplePresence;

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
