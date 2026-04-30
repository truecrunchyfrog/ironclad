use anyhow::bail;
use ulid::Ulid;

use crate::{args::fact::add::AddFactArgs, context::Context};

pub(crate) fn dispatch(context: &Context, args: AddFactArgs) -> anyhow::Result<()> {
    let mut session = context.catalog_session()?;

    let fact_id = Ulid::new().to_string();
    let path = session.catalog().fact_file_path(&fact_id);
    let label = args.label.clone();

    if let Some(label) = &label {
        if session
            .index_mut()
            .insert(label.clone(), fact_id.clone())
            .is_some()
        {
            bail!("label '{label}' already indexed");
        }
    }

    if let Err(err) = std::fs::write(&path, []) {
        if let Some(label) = &label {
            session.index_mut().remove_label(label);
        }
        return Err(err.into());
    }

    if let Some(label) = &label {
        if let Err(err) = session.save_index() {
            let _ = std::fs::remove_file(&path);
            session.index_mut().remove_label(label);
            return Err(err.into());
        }
    }

    println!("{}", label.unwrap_or(fact_id));

    Ok(())
}
