use std::io::Write;

use ironclad_core::{catalog::SnapshotProgressEvent, fact::RecipeProgressEvent};

use crate::{
    args::resolve::ResolveArgs,
    context::Context,
    helper::{SnapshotPath, write_snapshot},
};

pub(super) fn dispatch(context: &Context, args: ResolveArgs) -> anyhow::Result<()> {
    let session = context.catalog_session()?;

    let no_redact = args.no_redact;

    let facts = match args {
        ResolveArgs { include, .. } if !include.is_empty() => {
            session.labeled_facts_including(&include)?
        }
        ResolveArgs { exclude, .. } => session.labeled_facts_excluding(&exclude)?,
    };

    let total = facts.len();

    eprint!("...");

    let result_snapshot =
        session
            .catalog()
            .capture_snapshot(context.registry(), facts, !no_redact, |update| {
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
        session.catalog(),
        args.output,
        SnapshotPath::Actual,
        &snapshot,
    )?;

    Ok(())
}
