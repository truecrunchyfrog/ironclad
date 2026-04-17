use crate::{args::fact::list::ListFactArgs, config::Config, helper::resolve_catalog};

pub(crate) fn dispatch(_config: &Config, args: ListFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let facts = catalog.load_facts()?;

    for (label, path, fact) in facts {
        if args.verbose {
            println!(
                "{label}: {path:?}: {}",
                fact.description()
                    .clone()
                    .unwrap_or_else(|| String::from("-"))
            );
        } else {
            println!("{label}");
        }
    }

    Ok(())
}
