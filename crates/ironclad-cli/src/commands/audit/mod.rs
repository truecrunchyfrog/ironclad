use std::time::Duration;

use ironclad_core::snapshot::diff::SamplePresence;

use crate::{
    args::audit::AuditArgs,
    helper::{resolve_explicit_or_reused_cell_id, resolve_ledger},
    output, ui,
};

pub(super) fn dispatch(args: AuditArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let show_cell_ids = args
        .cell_id
        .into_iter()
        .map(|cell_id| resolve_explicit_or_reused_cell_id(&ledger, Some(cell_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let audit = match args {
        AuditArgs { new: true, .. } => ledger.capture_snapshot(None)?,
        AuditArgs { cache: true, .. } => ledger.load_pending_snapshot().unwrap_or_default(),
        _ => ledger.capture_snapshot(Some(ledger.load_pending_snapshot().unwrap_or_default()))?,
    };
    let baseline = ledger.load_baseline_snapshot().unwrap_or_default();
    let diffs = audit.diff(baseline);

    if !args.dry_run {
        ledger.save_pending_snapshot(audit)?;
    }

    let mut unacked_cells: usize = 0;
    let mut oldest_cache_age = Duration::ZERO;

    for (cell_id, (diff, dependency_diffs)) in diffs {
        let cache_age = diff
            .after()
            .as_ref()
            .map(|batch| batch.created().elapsed())
            .transpose()?;
        let sample_diffs = diff.sample_diffs();
        let mut attributes = Vec::<String>::new();
        let mut needs_resolve = false;

        if sample_diffs.iter().any(|(_, p)| p != &SamplePresence::Both) {
            needs_resolve = true;
            attributes.push(format!(
                "dirty ({})",
                output::display_dirtiness(sample_diffs.iter().map(|(_, p)| p).collect::<Vec<_>>())
            ));
        }

        let stale_deps = dependency_diffs
            .into_iter()
            .map(|(cell_id, diff)| (cell_id, diff.sample_diffs()))
            .filter(|(_, samples)| samples.iter().any(|(_, p)| p != &SamplePresence::Both))
            .collect::<Vec<_>>();

        if !stale_deps.is_empty() {
            needs_resolve = true;
            attributes.push(format!(
                "stale ({} across {})",
                output::display_dirtiness(
                    stale_deps
                        .iter()
                        .flat_map(|(_, p)| p)
                        .map(|(_, p)| p)
                        .collect::<Vec<_>>()
                ),
                stale_deps.len()
            ));
        }

        if let Some(age) = cache_age
            && age.as_secs() > 0
        {
            oldest_cache_age = oldest_cache_age.max(age);
            attributes.push(format!(
                "cache ({})",
                humantime::format_duration(Duration::from_secs(age.as_secs()))
            ));
        }

        if needs_resolve {
            unacked_cells += 1;
        }

        if needs_resolve && (show_cell_ids.is_empty() || show_cell_ids.contains(&cell_id)) {
            println!("{}: {}", cell_id, attributes.join(", "));

            if args.expand_diff {
                for sample_diff in sample_diffs
                    .iter()
                    .filter(|(_, p)| p != &SamplePresence::Both)
                {
                    println!("{}", output::display_sample_diff(sample_diff))
                }
            }

            if args.expand_stale {
                for stale_dep in stale_deps {
                    println!(
                        "    stale over {}: dirty ({})",
                        stale_dep.0,
                        output::display_dirtiness(
                            stale_dep.1.iter().map(|(_, p)| p).collect::<Vec<_>>()
                        )
                    );
                    if args.expand_diff {
                        for sample_diff in stale_dep
                            .1
                            .iter()
                            .filter(|(_, p)| p != &SamplePresence::Both)
                        {
                            println!("{}", output::display_sample_diff(sample_diff))
                        }
                    }
                }
            }
        }
    }

    if oldest_cache_age.as_secs() > 0 {
        ui::warn(format!(
            "cache is up to {} old",
            humantime::format_duration(Duration::from_secs(oldest_cache_age.as_secs()))
        ));
    }

    match unacked_cells {
        0 => println!("ok!"),
        amount => println!("{amount} not ack'd"),
    }

    Ok(())
}
