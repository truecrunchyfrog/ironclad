use std::process::Command;

use ironclad_core::catalog::Catalog;

use crate::{args::fact::edit::EditFactArgs, config::Config, helper::resolve_catalog};

pub(crate) fn dispatch(_config: &Config, args: EditFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let index = catalog.load_fact_index()?;
    let fact_id = Catalog::fact_id_for_label(&index, &args.label)?;
    let path = catalog.fact_file_path(&fact_id);

    Command::new(std::env::var("EDITOR")?)
        .arg(path.to_str().unwrap())
        .status()?;

    Ok(())
}
