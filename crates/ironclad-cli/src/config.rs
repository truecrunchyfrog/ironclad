use std::path::PathBuf;

use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Debug, Args, Serialize, Deserialize)]
pub(crate) struct Config {
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,

    #[arg(long)]
    pub(crate) config_file: Option<PathBuf>,

    #[arg(long)]
    pub(crate) catalog_dir: Option<PathBuf>,
}
