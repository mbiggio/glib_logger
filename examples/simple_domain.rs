//
// This example shows basic usage of custom, fixed domains
//
// To run this example, use:
//
//    cargo run --example simple_domain
//

extern crate glib_logger;
use std::env;
#[macro_use]
extern crate log;

static DOMAIN_LOGGER: glib_logger::Logger = glib_logger::custom(
    glib_logger::LoggerType::Simple,
    glib_logger::LoggerDomain::Custom("mydomain")
);

fn main() {
    env::set_var("G_MESSAGES_DEBUG", "all");

    glib_logger::init(&DOMAIN_LOGGER);
    log::set_max_level(log::LevelFilter::Debug);

    info!("info message: {}", 2);
    warn!("warning message: {}", "foobar");
    debug!("Hello, world!");
}
