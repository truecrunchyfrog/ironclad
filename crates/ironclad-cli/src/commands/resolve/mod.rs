use std::io::Write;

use ironclad_core::{
    catalog::{FactSelection, SnapshotFile, SnapshotProgressEvent},
    fact::RecipeProgressEvent,
};

use crate::{args::resolve::ResolveArgs, context::Context, helper::write_snapshot};

pub(super) fn dispatch(context: &Context, args: ResolveArgs) -> anyhow::Result<()> {
    let session = context.catalog_session()?;
    let selection = selection_from_args(&args);
    let total = session.labeled_facts(selection_from_args(&args))?.len();

    eprint!("...");

    let result_snapshot =
        session.capture_snapshot(context.registry(), selection, !args.no_redact, |update| {
            if let SnapshotProgressEvent::FactStep {
                index,
                fact,
                inner:
                    RecipeProgressEvent::StepStarted {
                        index: step_index,
                        step,
                        ..
                    },
                ..
            } = update
            {
                eprint!(
                    "\r\x1b[2K{}/{}: {}: {}/{}: {}",
                    index + 1,
                    total,
                    fact.label,
                    step_index + 1,
                    fact.steps().len(),
                    step.operation_id()
                );
                let _ = std::io::stderr().flush();
            }
        });

    eprint!("\r\x1b[2K");

    let snapshot = match result_snapshot {
        Ok(snapshot) => snapshot,
        Err(err) => return Err(err.into()),
    };

    write_snapshot(
        session.repository(),
        args.output,
        SnapshotFile::Actual,
        &snapshot,
    )?;

    Ok(())
}

fn selection_from_args(args: &ResolveArgs) -> FactSelection {
    match args {
        ResolveArgs { include, .. } if !include.is_empty() => {
            FactSelection::Include(include.clone())
        }
        ResolveArgs { exclude, .. } if !exclude.is_empty() => {
            FactSelection::Exclude(exclude.clone())
        }
        _ => FactSelection::All,
    }
}
