use ironclad_core::catalog::Catalog;

use crate::{args::fact::remove::RemoveFactArgs, config::Config, helper::resolve_catalog};

pub(crate) fn dispatch(_config: &Config, args: RemoveFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let fact_id = Catalog::fact_id_for_label(&catalog.load_fact_index()?, &args.label)?;
    std::fs::remove_file(catalog.fact_file_path(&fact_id))?;

    println!("{}", args.label);

    Ok(())
}
