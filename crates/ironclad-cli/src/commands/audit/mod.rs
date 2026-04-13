use std::time::Duration;

use console::style;
use ironclad_core::{fact::id::FactId, snapshot::diff::BatchDiff};

use crate::{
    args::audit::AuditArgs,
    batch_origin::BatchOrigin,
    config::Config,
    helper::{collect_changed_snapshot_diffs, resolve_cluster, resolve_explicit_or_reused_fact_id},
    output::format_batch_diff,
    ui,
};

pub(super) fn dispatch(_config: &Config, args: AuditArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;
    let show_fact_ids = args
        .fact_id
        .into_iter()
        .map(|fact_id| resolve_explicit_or_reused_fact_id(&cluster, Some(fact_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let audit = match args {
        AuditArgs { fresh: true, .. } => cluster.capture_snapshot(None)?,
        AuditArgs { cache: true, .. } => cluster.load_pending_snapshot().unwrap_or_default(),
        _ => cluster.capture_snapshot(Some(cluster.load_pending_snapshot().unwrap_or_default()))?,
    };
    let baseline = cluster.load_baseline_snapshot().unwrap_or_default();

    let relevant_diffs = collect_changed_snapshot_diffs(audit.diff(&baseline));

    if !args.dry_run {
        cluster.save_pending_snapshot(audit)?;
    }

    let batch_diff_count = relevant_diffs.len();
    let oldest_cache_age = relevant_diffs
        .iter()
        .flat_map(|(_, diff)| diff.after().as_ref().map(|batch| batch.created()))
        .max()
        .map(|time| time.elapsed())
        .transpose()?
        .unwrap_or(Duration::ZERO);

    for (origin, diff) in relevant_diffs {
        let cache_age = diff
            .after()
            .as_ref()
            .map(|batch| batch.created().elapsed())
            .transpose()?
            .filter(|created| created.as_secs() > 0);

        println!(
            "{} {}",
            format_batch_diff(&origin, &diff),
            style(
                cache_age
                    .map(|age| format!(
                        "cache {}",
                        humantime::format_duration(Duration::from_secs(age.as_secs()))
                    ))
                    .unwrap_or_default()
            )
            .yellow()
        );
    }

    if oldest_cache_age.as_secs() > 0 {
        ui::warn(format!(
            "cache is up to {} old",
            humantime::format_duration(Duration::from_secs(oldest_cache_age.as_secs()))
        ));
    }

    match batch_diff_count {
        0 => println!("{}", style("clean").green()),
        amount => println!("{} {amount} off baseline", style("dirty:").red()),
    }

    Ok(())
}
