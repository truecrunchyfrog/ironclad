use std::process::Command;

use anyhow::{Context, anyhow, bail};

use crate::{args::fact::edit::EditFactArgs, config::Config, helper::CatalogSession};

pub(crate) fn dispatch(_config: &Config, args: EditFactArgs) -> anyhow::Result<()> {
    let session = CatalogSession::open()?;
    let resolved = session.resolve_fact_ref(&args.selector)?;
    let path = session.catalog().fact_file_path(&resolved.fact_id);
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
