use crate::{
    args::fact::remove::RemoveFactArgs,
    config::Config,
    helper::{resolve_catalog, resolve_fact},
};

pub(crate) fn dispatch(_config: &Config, args: RemoveFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let resolved = resolve_fact(&catalog, &args.selector)?;
    let mut index = catalog.load_fact_index()?;

    std::fs::remove_file(catalog.fact_file_path(&resolved.fact_id))?;

    index
        .entries_mut()
        .retain(|_, fact_id| fact_id != &resolved.fact_id);
    catalog.save_fact_index(&index)?;

    println!("{}", resolved.selector);

    Ok(())
}
