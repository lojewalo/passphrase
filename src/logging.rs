use log::{LevelFilter, Log, Metadata, Record};

pub fn set_up() {
    if let Err(e) = log::set_logger(&Logger).map(|()| log::set_max_level(LevelFilter::Info)) {
        eprintln!("could not set up logger: {}", e);
    }
}

struct Logger;

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        eprintln!("{} - {}", record.level(), record.args());
    }

    fn flush(&self) {
        // no-op
    }
}
