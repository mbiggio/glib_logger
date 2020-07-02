//
// This example shows basic usage of a custom log function in glib for simple
// (non-structured) log formats. This is actually all done by the `glib_sys`
// crate, but example is included here because it's relevant to the logging
// use-case.
//
// To run this example, use:
//
//    cargo run --example custom_log_function
//

extern crate glib_logger;
use std::env;
#[macro_use]
extern crate log;
use std::os::raw::c_char;

static DOMAIN_LOGGER: glib_logger::Logger = glib_logger::custom(
    glib_logger::LoggerType::Simple,
    glib_logger::LoggerDomain::Custom("mydomain")
);

fn main() {
    env::set_var("G_MESSAGES_DEBUG", "all");

    // setup the custom log handler
    unsafe {
        glib_sys::g_log_set_default_handler(
            Some(log_handler),    // our "extern" function to perform logging
            std::ptr::null_mut()  // a pointer to optional "user data"
        );
    }

    glib_logger::init(&DOMAIN_LOGGER);
    log::set_max_level(log::LevelFilter::Debug);

    info!("info message: {}", 2);
    warn!("warning message: {}", "foobar");
    debug!("Hello, world!");
}

unsafe extern "C" fn log_handler(
    domain_ptr: *const c_char,
    level: glib_sys::GLogLevelFlags,
    message_ptr: *const c_char,
    _data_ptr: glib_sys::gpointer,
) {
    use std::ffi::CStr;

    let message = if !message_ptr.is_null() {
        Some(CStr::from_ptr(message_ptr).to_string_lossy().into_owned())
    } else {
        None
    };

    let domain = if !domain_ptr.is_null() {
        Some(CStr::from_ptr(domain_ptr).to_string_lossy().into_owned())
    } else {
        None
    };

    println!("LOG: {:?} - {:?} - {:?}", level, domain, message);
}

