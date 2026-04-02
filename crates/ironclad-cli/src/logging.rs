use log::LevelFilter;

use crate::config::Config;

pub(crate) fn init(config: &Config) {
    let level = match config.verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    env_logger::builder().filter_level(level).init();
}
