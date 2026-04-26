mod args;
mod commands;
pub(crate) mod config;
mod context;
pub(crate) mod helper;
mod logging;

use std::{env::home_dir, process::ExitCode};

use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use ironclad_core::registry::Registry;
use ironclad_ops::register_ops;

use crate::{config::Config, context::Context};

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
        .or_else(|| home_dir().map(|home_dir| home_dir.join(".config/ironclad/config.toml")));

    if let Some(file) = config_file {
        figment_builder = figment_builder.merge(Toml::file(file));
    }

    let config: Config = figment_builder.extract()?;

    logging::init(&config);

    let mut registry = Registry::new();
    register_ops(&mut registry)?;

    let context = Context::new(config, registry);

    commands::dispatch(&context, cli.command)?;

    Ok(())
}
