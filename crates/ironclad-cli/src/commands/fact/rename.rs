use anyhow::bail;

use crate::{
    args::fact::rename::RenameFactArgs,
    config::Config,
    helper::{resolve_catalog, resolve_fact},
};

pub(crate) fn dispatch(_config: &Config, args: RenameFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let resolved = resolve_fact(&catalog, &args.selector)?;
    let mut index = catalog.load_fact_index()?;
    let entries = index.entries_mut();

    if entries
        .iter()
        .any(|(label, fact_id)| label == &args.new_label && fact_id != &resolved.fact_id)
    {
        bail!("label '{}' already indexed", args.new_label);
    }

    entries.retain(|_, fact_id| fact_id != &resolved.fact_id);
    entries.insert(args.new_label.clone(), resolved.fact_id);

    catalog.save_fact_index(&index)?;

    println!("{}", args.new_label);

    Ok(())
}
