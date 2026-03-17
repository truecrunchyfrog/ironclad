use log::LevelFilter;

use crate::args::Cli;

pub(crate) fn init(cli: &Cli) {
    let level = match cli.global.verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    env_logger::builder().filter_level(level).init();
}
