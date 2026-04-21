use std::collections::HashMap;

use anyhow::anyhow;
use ironclad_core::{fact::Fact, recipe::Recipe};
use ulid::Ulid;

use crate::{args::fact::add::AddFactArgs, config::Config, helper::resolve_catalog};

pub(crate) fn dispatch(_config: &Config, args: AddFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let fact = Fact::new(
        args.description,
        Vec::new(),
        HashMap::new(),
        Recipe::default(),
        args.secret,
    );

    let fact_id = Ulid::new().to_string();

    let path = catalog.fact_file_path(&fact_id);

    if let Some(label) = &args.label {
        let mut index = catalog.load_fact_index()?;
        let entries = index.entries_mut();
        if entries.insert(label.clone(), fact_id.clone()).is_some() {
            return Err(anyhow!("label '{label}' already indexed"));
        }
        catalog.save_fact_index(&index)?;
    }

    std::fs::write(path, serde_json::to_vec_pretty(&fact)?)?;

    println!("{}", args.label.unwrap_or(fact_id));

    Ok(())
}
