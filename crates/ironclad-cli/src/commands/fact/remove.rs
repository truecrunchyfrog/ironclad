use crate::{args::fact::remove::RemoveFactArgs, config::Config, helper::CatalogSession};

pub(crate) fn dispatch(_config: &Config, args: RemoveFactArgs) -> anyhow::Result<()> {
    let mut session = CatalogSession::open()?;
    let resolved = session.resolve_fact_ref(&args.selector)?;

    std::fs::remove_file(session.catalog().fact_file_path(&resolved.fact_id))?;

    session.index_mut().remove_fact_id(&resolved.fact_id);
    session.save_index()?;

    println!("{}", resolved.selector);

    Ok(())
}
