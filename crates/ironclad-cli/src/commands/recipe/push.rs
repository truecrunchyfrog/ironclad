use ironclad_core::{catalog::Catalog, recipe::Step, registry};

use crate::{args::recipe::push::PushRecipeArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: PushRecipeArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let index = catalog.load_fact_index()?;
    let fact_id = Catalog::fact_id_for_label(&index, &args.label)?;
    let path = catalog.fact_file_path(&fact_id);
    let mut fact = catalog.load_fact_for_path(&path)?;

    registry::resolve_op(&args.operation_id)?;

    let options = match args.options {
        Some(serialized) => serde_json::from_str::<serde_json::Value>(&serialized)?,
        None => serde_json::Value::Null,
    };

    let step = Step::new(args.operation_id, options);

    fact.steps_mut().add(args.index, step)?;

    std::fs::write(path, serde_json::to_vec_pretty(&fact)?)?;

    Ok(())
}
