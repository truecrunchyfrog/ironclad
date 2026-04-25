mod args;
mod commands;
pub(crate) mod config;
pub(crate) mod helper;
mod logging;
pub(crate) mod output;

use std::{env::home_dir, process::ExitCode};

use figment::{
    Figment,
    providers::{Env, Format, Json, Serialized},
};
use ironclad_ops::register_ops;

use crate::config::Config;

fn main() -> ExitCode {
    match start() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("fatal: {err}");
            ExitCode::FAILURE
        }
    }
}

fn start() -> anyhow::Result<()> {
    let cli = args::parse();

    let mut figment_builder = Figment::new()
        .merge(Serialized::defaults(&cli.config))
        .merge(Env::prefixed("IC_"));

    let config_file = figment_builder
        .extract::<Config>()?
        .config_file
        .clone()
        .or_else(|| home_dir().map(|home_dir| home_dir.join(".config/ironclad/config.json")));

    if let Some(file) = config_file {
        figment_builder = figment_builder.merge(Json::file(file));
    }

    let config: Config = figment_builder.extract()?;

    logging::init(&config);

    register_ops()?;

    commands::dispatch(&config, cli.command)?;

    Ok(())
}
