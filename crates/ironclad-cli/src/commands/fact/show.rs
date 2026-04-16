use ironclad_core::fact::id::FactId;

use crate::{args::fact::show::ShowFactArgs, config::Config, helper::resolve_catalog};

pub(crate) fn dispatch(_config: &Config, args: ShowFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let fact = catalog.load_fact_for_id(&FactId::from(args.fact_id))?;

    match args {
        ShowFactArgs { raw: true, .. } => {
            println!("{}", serde_json::to_string_pretty(&fact)?);
        }

        ShowFactArgs { path: true, .. } => {
            println!("{}", catalog.fact_file_path(fact.id()).to_string_lossy());
        }

        _ => {
            println!(
                "{}\ndescription: {}\nsteps: {}",
                fact.id(),
                fact.description()
                    .clone()
                    .unwrap_or_else(|| String::from("none")),
                fact.recipe().steps().len()
            );
        }
    }

    Ok(())
}
