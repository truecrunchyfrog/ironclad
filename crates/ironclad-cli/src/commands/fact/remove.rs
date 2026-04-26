use crate::{args::fact::remove::RemoveFactArgs, context::Context};

pub(crate) fn dispatch(context: &Context, args: RemoveFactArgs) -> anyhow::Result<()> {
    let mut session = context.catalog_session()?;
    let resolved = session.resolve_fact_ref(&args.selector)?;

    std::fs::remove_file(session.catalog().fact_file_path(&resolved.fact_id))?;

    session.index_mut().remove_fact_id(&resolved.fact_id);
    session.save_index()?;

    println!("{}", resolved.selector);

    Ok(())
}
