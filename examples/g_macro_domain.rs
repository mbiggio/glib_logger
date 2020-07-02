//
// This example shows basic usage of custom domains using a G_LOG_DOMAIN
// variable and `g_trace` .. `g_error` macros. The variable should be static,
// but it can be different for each module.
//
// To run this example, use:
//
//    cargo run --example g_macro_domain
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

    g_info!("info message: {}", 2);
    g_warn!(target: "my-custom-domain", "warning message: {}", "foobar");
    different_domain::log_on_different_domain();
    g_debug!("Hello, world!");
}

mod different_domain {
    static G_LOG_DOMAIN: &str = "my-scoped-domain";
    pub fn log_on_different_domain() {
        g_error!("this will be in a scoped domain");
    }
}
