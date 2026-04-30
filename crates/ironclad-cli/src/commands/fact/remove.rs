use crate::{args::fact::remove::RemoveFactArgs, context::Context};

pub(crate) fn dispatch(context: &Context, args: RemoveFactArgs) -> anyhow::Result<()> {
    let mut session = context.catalog_session()?;
    let resolved = session.resolve_fact_ref(&args.selector)?;
    let label = session.index().label_for_id(&resolved.fact_id).map(str::to_string);
    let path = session.catalog().fact_file_path(&resolved.fact_id);

    if let Some(label) = &label {
        session.index_mut().remove_label(label);
        session.save_index()?;
    }

    if let Err(err) = std::fs::remove_file(&path) {
        if let Some(label) = label {
            session.index_mut().insert(label, resolved.fact_id.clone());
            let _ = session.save_index();
        }
        return Err(err.into());
    }

    println!("{}", resolved.selector);

    Ok(())
}
