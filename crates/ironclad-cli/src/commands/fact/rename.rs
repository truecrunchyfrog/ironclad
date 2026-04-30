use anyhow::bail;

use crate::{args::fact::rename::RenameFactArgs, context::Context};

pub(crate) fn dispatch(context: &Context, args: RenameFactArgs) -> anyhow::Result<()> {
    let mut session = context.catalog_session()?;
    let resolved = session.resolve_fact_ref(&args.selector)?;
    let old_label = session.index().label_for_id(&resolved.fact_id).map(str::to_string);

    let Some(old_label) = old_label else {
        bail!("cannot rename unindexed fact: {}", resolved.fact_id);
    };

    if session.index().id_for_label(&args.new_label) != Some(resolved.fact_id.as_str())
        && session.index().contains_label(&args.new_label)
    {
        bail!("label '{}' already indexed", args.new_label);
    }

    session.index_mut().remove_label(&old_label);
    session
        .index_mut()
        .insert(args.new_label.clone(), resolved.fact_id);

    session.save_index()?;

    println!("{}", args.new_label);

    Ok(())
}
