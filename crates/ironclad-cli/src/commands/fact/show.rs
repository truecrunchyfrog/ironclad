use ironclad_core::catalog::Catalog;

use crate::{args::fact::show::ShowFactArgs, config::Config, helper::resolve_catalog};

pub(crate) fn dispatch(_config: &Config, args: ShowFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let index = catalog.load_fact_index()?;
    let fact_id = Catalog::fact_id_for_label(&index, &args.label)?;
    let path = catalog.fact_file_path(&fact_id);
    let fact = catalog.load_fact_for_path(&path)?;

    if args.path {
        println!("{}", path.to_string_lossy());
    } else {
        println!("{}", fact.description().clone().unwrap_or_default());
    }

    Ok(())
}
