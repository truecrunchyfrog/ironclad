use std::time::Duration;

use anyhow::anyhow;
use ironclad_core::catalog::Catalog;

use crate::{args::fact::edit::EditFactArgs, config::Config, helper::resolve_catalog};

pub(crate) fn dispatch(_config: &Config, args: EditFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let mut index = catalog.load_fact_index()?;
    let fact_id = Catalog::fact_id_for_label(&index, &args.label)?;
    let path = catalog.fact_file_path(&fact_id);
    let mut fact = catalog.load_fact_for_path(&path)?;

    if let Some(description) = args.description {
        *fact.description_mut() = Some(description);
    }

    if args.unset_description {
        *fact.description_mut() = None;
    }

    if let Some(cache_lifespan) = args.cache_lifespan {
        *fact.cache_lifespan_mut() = cache_lifespan.into();
    }

    if args.unset_cache_lifespan {
        *fact.cache_lifespan_mut() = Duration::ZERO;
    }

    if let Some(new_label) = &args.relabel {
        let entries = index.entries_mut();

        if entries.insert(new_label.clone(), fact_id).is_some() {
            return Err(anyhow!("label '{new_label}' already indexed"));
        }

        entries
            .remove(&args.label)
            .expect("fact label should exist as index entry");

        catalog.save_fact_index(&index)?;
    }

    std::fs::write(path, serde_json::to_vec_pretty(&fact)?)?;

    println!("{}", args.relabel.unwrap_or(args.label));

    Ok(())
}
