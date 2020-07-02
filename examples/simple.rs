//
// This example shows basic usage of the glib logging library
//
// To run this example, use:
//
//    cargo run --example simple
//

extern crate glib_logger;
#[macro_use]
extern crate log;
use std::env;

fn main() {
    env::set_var("G_MESSAGES_DEBUG", "all");

    glib_logger::init(&glib_logger::SIMPLE);
    log::set_max_level(log::LevelFilter::Debug);

    info!("info message: {}", 2);
    warn!("warning message: {}", "foobar");
    debug!("Hello, world!");
}
