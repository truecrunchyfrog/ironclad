use anyhow::bail;
use ironclad_core::snapshot::Snapshot;

use crate::{
    args::apply::ApplyArgs,
    context::Context,
    helper::{SnapshotPath, read_snapshot, write_snapshot},
};

pub(super) fn dispatch(context: &Context, args: ApplyArgs) -> anyhow::Result<()> {
    let session = context.catalog_session()?;
    let promotion = read_snapshot(session.catalog(), args.promotion, SnapshotPath::Actual)?;
    let baseline = read_snapshot(session.catalog(), args.baseline, SnapshotPath::Canon)?;

    let promoted_baseline = match args {
        ApplyArgs { all: true, .. } => promotion,
        ApplyArgs {
            all: false,
            label: labels,
            ..
        } => {
            let mut baseline_entries = baseline.into_entries();
            let mut promotion_entries = promotion.into_entries();

            for label in labels {
                if baseline_entries.remove(&label).is_none()
                    && !promotion_entries.contains_key(&label)
                {
                    bail!("absent from proposal and baseline: {label}");
                }

                if let Some(entry) = promotion_entries.remove(&label) {
                    baseline_entries.insert(label, entry);
                }
            }

            Snapshot::new(baseline_entries)
        }
    };

    write_snapshot(
        session.catalog(),
        args.output,
        SnapshotPath::Canon,
        &promoted_baseline,
    )?;

    Ok(())
}
