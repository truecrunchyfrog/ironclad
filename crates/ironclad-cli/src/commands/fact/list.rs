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

    let labeled_fact_ids = labeled_entries
        .into_iter()
        .map(|(_, fact_id)| fact_id)
        .collect::<std::collections::HashSet<_>>();

    let mut unlabeled_fact_ids = std::fs::read_dir(catalog.facts_dir_path())?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("toml"))
        .filter_map(|path| {
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .map(ToString::to_string)
        })
        .filter(|fact_id| !labeled_fact_ids.contains(fact_id))
        .collect::<Vec<_>>();
    unlabeled_fact_ids.sort();

    for fact_id in unlabeled_fact_ids {
        if args.verbose {
            let fact = catalog.load_fact_for_path(&catalog.fact_file_path(&fact_id))?;
            println!(
                "{fact_id}: {}",
                fact.description()
                    .clone()
                    .unwrap_or_else(|| String::from("-"))
            );
        } else {
            println!("{fact_id}");
        }
    }

    Ok(())
}
