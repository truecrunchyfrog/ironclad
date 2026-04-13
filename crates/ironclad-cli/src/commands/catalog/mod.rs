mod init;

use crate::{args::catalog::CatalogCommand, config::Config};

pub(super) fn dispatch(config: &Config, command: CatalogCommand) -> anyhow::Result<()> {
    match command {
        CatalogCommand::Init(args) => init::dispatch(config, args),
    }
}
