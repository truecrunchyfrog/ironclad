use crate::{args::fact::list::ListFactArgs, context::Context};

pub(crate) fn dispatch(context: &Context, args: ListFactArgs) -> anyhow::Result<()> {
    let session = context.catalog_session()?;

    for (label, fact_id) in session.index().iter() {
        if args.verbose {
            let fact = session
                .catalog()
                .load_fact_for_path(&session.catalog().fact_file_path(fact_id))?;

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
