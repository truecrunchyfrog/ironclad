use crate::{args::fact::show::ShowFactArgs, context::Context};

pub(crate) fn dispatch(context: &Context, args: ShowFactArgs) -> anyhow::Result<()> {
    let session = context.catalog_session()?;
    let resolved = session.resolve_fact_ref(&args.selector)?;
    let path = session.catalog().fact_file_path(&resolved.fact_id);

    if args.path {
        println!("{}", path.to_string_lossy());
    } else {
        let fact = session.load_fact(&resolved.fact_id)?;
        println!("{}", fact.description().clone().unwrap_or_default());
    }

    Ok(())
}
