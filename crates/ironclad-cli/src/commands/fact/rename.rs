use anyhow::bail;
use ironclad_core::catalog::Catalog;

use crate::{args::fact::rename::RenameFactArgs, config::Config, helper::resolve_catalog};

pub(crate) fn dispatch(_config: &Config, args: RenameFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let mut index = catalog.load_fact_index()?;
    let fact_id = Catalog::fact_id_for_label(&index, &args.label)?;

    let entries = index.entries_mut();

    if entries.insert(args.new_label.clone(), fact_id).is_some() {
        bail!("label '{}' already indexed", args.new_label);
    }

    entries
        .remove(&args.label)
        .expect("fact label should exist as index entry");

    catalog.save_fact_index(&index)?;

    println!("{}", args.new_label);

    Ok(())
}
