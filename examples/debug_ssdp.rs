extern crate log;
extern crate ssdp;

use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

use ssdp::header::{HeaderMut, Man, MX, ST};
use ssdp::message::{Multicast, SearchRequest};

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

fn main() {
    init_logger().unwrap();

    // Create Our Search Request
    let mut request = SearchRequest::new();

    // Set Our Desired Headers (Not Verified By The Library)
    request.set(Man);
    request.set(MX(5));
    request.set(ST::All);

    // Collect Our Responses
    request.multicast().unwrap().into_iter().collect::<Vec<_>>();
}
