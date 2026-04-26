use std::process::Command;

use anyhow::{Context, anyhow, bail};

use ironclad_core::catalog::Catalog;

use crate::{args::fact::edit::EditFactArgs, config::Config, helper::resolve_catalog};

pub(crate) fn dispatch(_config: &Config, args: EditFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let index = catalog.load_fact_index()?;
    let fact_id = Catalog::fact_id_for_label(&index, &args.label)?;
    let path = catalog.fact_file_path(&fact_id);
    let editor = std::env::var("EDITOR").context("$EDITOR is not set")?;
    let argv = shlex::split(&editor).ok_or_else(|| anyhow!("failed to parse $EDITOR"))?;

    if argv.is_empty() {
        bail!("$EDITOR is empty");
    }

    let status = Command::new(&argv[0])
        .args(&argv[1..])
        .arg(&path)
        .status()
        .with_context(|| format!("failed to launch editor '{}'", argv[0]))?;

    if let Some(code) = status.code() {
        if code == 0 {
            Ok(())
        } else {
            std::process::exit(code);
        }
    } else {
        bail!("editor terminated by signal");
    }
}
