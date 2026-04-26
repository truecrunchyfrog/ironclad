use anyhow::bail;

use crate::{args::fact::rename::RenameFactArgs, config::Config, helper::CatalogSession};

pub(crate) fn dispatch(_config: &Config, args: RenameFactArgs) -> anyhow::Result<()> {
    let mut session = CatalogSession::open()?;
    let resolved = session.resolve_fact_ref(&args.selector)?;

    if session.index().id_for_label(&args.new_label) != Some(resolved.fact_id.as_str())
        && session.index().contains_label(&args.new_label)
    {
        bail!("label '{}' already indexed", args.new_label);
    }

    session.index_mut().remove_fact_id(&resolved.fact_id);
    session
        .index_mut()
        .insert(args.new_label.clone(), resolved.fact_id);

    session.save_index()?;

    println!("{}", args.new_label);

    Ok(())
}
