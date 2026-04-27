use std::path::PathBuf;

use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Debug, Args, Serialize, Deserialize)]
pub(crate) struct Config {
    /// Increase log verbosity.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,

    /// Read configuration from this file.
    #[arg(long)]
    pub(crate) config_file: Option<PathBuf>,

    /// Use this exact catalog directory.
    #[arg(long)]
    pub(crate) catalog_dir: Option<PathBuf>,
}
