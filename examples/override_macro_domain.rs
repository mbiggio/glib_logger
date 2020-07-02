//
// This example shows basic usage of custom domains using a G_LOG_DOMAIN
// variable while keeping, at the same time, usage of standard `log`-like
// macros. The variable should be static, but it can be different for each
// module.
//
// If you use similar code in your example, you'll need to enable the
// `redefine_log_macros` feature in `Cargo.toml`.
//
// To run this example, use:
//
//    cargo run --example override_macro_domain --features="redefine_log_macros"
//

#[macro_use]
extern crate glib_logger;

use std::env;

static DOMAIN_LOGGER: glib_logger::Logger = glib_logger::custom(
    glib_logger::LoggerType::Simple,
    glib_logger::LoggerDomain::Target
);

static G_LOG_DOMAIN: &str = "my-global-domain";

fn main() {
    env::set_var("G_MESSAGES_DEBUG", "all");

    glib_logger::init(&DOMAIN_LOGGER);
    log::set_max_level(log::LevelFilter::Debug);

    info!("info message: {}", 2);
    warn!(target: "my-custom-domain", "warning message: {}", "foobar");
    different_domain::log_on_different_domain();
    debug!("Hello, world!");
}

mod different_domain {
    static G_LOG_DOMAIN: &str = "my-scoped-domain";
    pub fn log_on_different_domain() {
        error!("this will be in a scoped domain");
    }
}
