use anyhow::bail;
use ulid::Ulid;

use crate::{args::fact::add::AddFactArgs, config::Config, helper::CatalogSession};

pub(crate) fn dispatch(_config: &Config, args: AddFactArgs) -> anyhow::Result<()> {
    let mut session = CatalogSession::open()?;

    let fact_id = Ulid::new().to_string();

    let path = session.catalog().fact_file_path(&fact_id);

    if let Some(label) = &args.label {
        if session
            .index_mut()
            .insert(label.clone(), fact_id.clone())
            .is_some()
        {
            bail!("label '{label}' already indexed");
        }
        session.save_index()?;
    }

    std::fs::write(path, [])?;

    println!("{}", args.label.unwrap_or(fact_id));

    Ok(())
}
