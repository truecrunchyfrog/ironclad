use console::style;

pub fn error(msg: impl AsRef<str>) {
    eprintln!("{} {}", style("error:").red(), msg.as_ref());
}

pub fn warn(msg: impl AsRef<str>) {
    eprintln!("{} {}", style("warn:").yellow(), msg.as_ref());
}

pub fn info(msg: impl AsRef<str>) {
    eprintln!("{} {}", style("info:").cyan(), msg.as_ref());
}
