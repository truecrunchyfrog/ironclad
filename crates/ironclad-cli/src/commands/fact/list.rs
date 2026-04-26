use crate::{args::fact::list::ListFactArgs, config::Config, helper::resolve_catalog};

pub(crate) fn dispatch(_config: &Config, args: ListFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let index = catalog.load_fact_index()?;
    let mut labeled_entries = index.into_entries().into_iter().collect::<Vec<_>>();
    labeled_entries.sort_by(|a, b| a.0.cmp(&b.0));

    for (label, fact_id) in &labeled_entries {
        if args.verbose {
            let fact = catalog.load_fact_for_path(&catalog.fact_file_path(fact_id))?;

            println!(
                "{label}: {}",
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
