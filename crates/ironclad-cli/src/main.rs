mod args;
mod commands;
pub(crate) mod helper;
mod logging;
pub(crate) mod output;
pub(crate) mod reuse_node;
pub(crate) mod ui;

use std::process::ExitCode;

use rnacl_ops::register_ops;

fn main() -> ExitCode {
    match start() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            ui::error(format!("{}", err));
            ExitCode::FAILURE
        }
    }
}

fn start() -> anyhow::Result<()> {
    let cli = args::parse();
    logging::init(&cli);
    register_ops()?;
    commands::dispatch(cli)?;
    Ok(())
}
