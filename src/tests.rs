use log::Level;
use std::os::raw::c_char;
use serial_test::serial;
use crate::*;

static TEST_LOGGER: InterchangableLoggerWrap = InterchangableLoggerWrap {
    wrapped_logger: std::cell::Cell::new(None)
};

struct InterchangableLoggerWrap {
    wrapped_logger: std::cell::Cell<Option<&'static dyn log::Log>>,
}

impl InterchangableLoggerWrap {
    pub fn set_wrapped_logger(&self, wrapped_logger: &'static dyn log::Log) {
        self.wrapped_logger.set(Some(wrapped_logger));
    }

    pub fn clear_wrapped_logger(&self) {
        self.wrapped_logger.set(None);
    }
}

unsafe impl Send for InterchangableLoggerWrap {}
unsafe impl Sync for InterchangableLoggerWrap {}

impl log::Log for InterchangableLoggerWrap {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        self.wrapped_logger.get().is_some()
    }

    fn log(&self, record: &log::Record) {
        if let Some(logger) = self.wrapped_logger.get() {
            logger.log(record);
        }
    }

    fn flush(&self) {}
}

struct LogTrace {
    domain: Option<String>,
    level: Option<glib_sys::GLogLevelFlags>,
    message: Option<String>,
}

impl LogTrace {
    fn new() -> LogTrace {
        LogTrace {
            domain: None,
            level: None,
            message: None,
        }
    }
}

fn collect_log(test_case: fn()) -> LogTrace {
    use glib_sys::g_log_set_default_handler;
    use std::ffi::c_void;
    use std::ptr;

    let mut trace = LogTrace::new();
    let prev_handler: glib_sys::GLogFunc;

    unsafe {
        prev_handler =
            g_log_set_default_handler(Some(log_writer), &mut trace as *mut _ as *mut c_void);
    }
    test_case();
    unsafe {
        g_log_set_default_handler(prev_handler, ptr::null_mut());
    }

    trace
}

unsafe extern "C" fn log_writer(
    domain_ptr: *const c_char,
    level: glib_sys::GLogLevelFlags,
    message_ptr: *const c_char,
    data_ptr: glib_sys::gpointer,
) {
    use std::ffi::CStr;

    if data_ptr.is_null() {
        panic!("own data is NULL");
    }
    let trace: &mut LogTrace = &mut *(data_ptr as *mut LogTrace);

    if !message_ptr.is_null() {
        trace.message = Some(CStr::from_ptr(message_ptr).to_string_lossy().into_owned());
    }
    if !domain_ptr.is_null() {
        trace.domain = Some(CStr::from_ptr(domain_ptr).to_string_lossy().into_owned());
    }
    trace.level = Some(level);
}

fn init_test_logger() {
    let _ = log::set_logger(&TEST_LOGGER);
    log::set_max_level(log::LevelFilter::Debug);
}

#[test]
#[serial]
fn simple_log() {
    let trace = collect_log(|| {
        logging::glib_log(None, Level::Debug, "foobar");
    });
    assert_eq!(trace.domain, None);
    assert_eq!(trace.message, Some("foobar".to_string()));
    assert_eq!(trace.level, Some(glib_sys::G_LOG_LEVEL_DEBUG));
}

#[test]
#[serial]
fn domain_log() {
    let trace = collect_log(|| {
        logging::glib_log(Some("barbaz"), Level::Debug, "foobar");
    });
    assert_eq!(trace.domain, Some(String::from("barbaz")));
    assert_eq!(trace.message, Some("foobar".to_string()));
    assert_eq!(trace.level, Some(glib_sys::G_LOG_LEVEL_DEBUG));
}


#[test]
#[serial]
fn simple_formatted_log() {
    let trace = collect_log(|| {
        logging::glib_log(
            None,
            Level::Info,
            &format!("this is a test {} \"{}\" %%d", 123, "abcd"),
        );
    });
    assert_eq!(trace.domain, None);
    assert_eq!(
        trace.message,
        Some("this is a test 123 \"abcd\" %d".to_string())
    );
    assert_eq!(trace.level, Some(glib_sys::G_LOG_LEVEL_INFO));
}

// TODO: figure out a way to install handler for structure logs

#[test]
#[serial]
fn via_logger() {
    let trace = collect_log(|| {
        use log::Log;

        let l = super::simple();
        l.log(&log::Record::builder()
            .level(Level::Error)
            .file(Some("foo.rs"))
            .line(Some(123))
            .module_path(None)
            .args(format_args!("this is a test \"{}\" {}", "abcd", 12))
            .build());
    });
    assert_eq!(trace.domain, None);
    assert_eq!(
        trace.message,
        Some("foo.rs:123: this is a test \"abcd\" 12".to_string())
    );
    assert_eq!(trace.level, Some(glib_sys::G_LOG_LEVEL_CRITICAL));
}

#[test]
#[serial]
fn via_macro() {
    static THIS_LOGGER: crate::Logger = crate::custom(crate::LoggerType::SimplePlain,
        crate::LoggerDomain::None);
    init_test_logger();
    TEST_LOGGER.set_wrapped_logger(&THIS_LOGGER);

    let trace = collect_log(|| {
        log::warn!("foobar");
    });
    assert_eq!(trace.domain, None);
    assert_eq!(trace.message, Some("foobar".to_string()));
    assert_eq!(trace.level, Some(glib_sys::G_LOG_LEVEL_WARNING));

    TEST_LOGGER.clear_wrapped_logger();
}

#[test]
#[serial]
fn via_macro_domain_default() {
    static THIS_LOGGER: crate::Logger = crate::custom(crate::LoggerType::SimplePlain,
        crate::LoggerDomain::Custom("barbaz"));
    init_test_logger();
    TEST_LOGGER.set_wrapped_logger(&THIS_LOGGER);

    let trace = collect_log(|| {
        log::warn!("foobar");
    });
    assert_eq!(trace.domain, Some("barbaz".to_string()));
    assert_eq!(trace.message, Some("foobar".to_string()));
    assert_eq!(trace.level, Some(glib_sys::G_LOG_LEVEL_WARNING));

    TEST_LOGGER.clear_wrapped_logger();
}

#[test]
#[serial]
fn via_macro_domain_target() {
    static THIS_LOGGER: crate::Logger = crate::custom(
        crate::LoggerType::SimplePlain, crate::LoggerDomain::Target);
    init_test_logger();
    TEST_LOGGER.set_wrapped_logger(&THIS_LOGGER);

    let trace = collect_log(|| {
        log::warn!(target: "notbarbaz", "foobar");
    });
    assert_eq!(trace.domain, Some("notbarbaz".to_string()));
    assert_eq!(trace.message, Some("foobar".to_string()));
    assert_eq!(trace.level, Some(glib_sys::G_LOG_LEVEL_WARNING));

    TEST_LOGGER.clear_wrapped_logger();
}

#[test]
#[serial]
fn via_gmacro_domain_target() {
    static THIS_LOGGER: crate::Logger = crate::custom(
        crate::LoggerType::SimplePlain, crate::LoggerDomain::Target);
    init_test_logger();
    TEST_LOGGER.set_wrapped_logger(&THIS_LOGGER);

    let trace = collect_log(|| {
        g_warn!(target: "notbarbaz", "foobar");
    });
    assert_eq!(trace.domain, Some("notbarbaz".to_string()));
    assert_eq!(trace.message, Some("foobar".to_string()));
    assert_eq!(trace.level, Some(glib_sys::G_LOG_LEVEL_WARNING));

    TEST_LOGGER.clear_wrapped_logger();
}

#[test]
#[serial]
fn via_gmacro_domain_localdomain() {
    static THIS_LOGGER: crate::Logger = crate::custom(
        crate::LoggerType::SimplePlain, crate::LoggerDomain::Target);
    static G_LOG_DOMAIN: &str = "barbaz";
    init_test_logger();
    TEST_LOGGER.set_wrapped_logger(&THIS_LOGGER);

    let trace = collect_log(|| {
        g_warn!("foobar");
    });
    assert_eq!(trace.domain, Some("barbaz".to_string()));
    assert_eq!(trace.message, Some("foobar".to_string()));
    assert_eq!(trace.level, Some(glib_sys::G_LOG_LEVEL_WARNING));

    TEST_LOGGER.clear_wrapped_logger();
}

#[test]
#[serial]
fn via_alias_macro_domain_target() {
    static THIS_LOGGER: crate::Logger = crate::custom(
        crate::LoggerType::SimplePlain, crate::LoggerDomain::Target);
    init_test_logger();
    TEST_LOGGER.set_wrapped_logger(&THIS_LOGGER);

    let trace = collect_log(|| {
        warn!(target: "notbarbaz", "foobar");
    });
    assert_eq!(trace.domain, Some("notbarbaz".to_string()));
    assert_eq!(trace.message, Some("foobar".to_string()));
    assert_eq!(trace.level, Some(glib_sys::G_LOG_LEVEL_WARNING));

    TEST_LOGGER.clear_wrapped_logger();
}

#[test]
#[serial]
fn via_alias_macro_domain_localdomain() {
    static THIS_LOGGER: crate::Logger = crate::custom(
        crate::LoggerType::SimplePlain, crate::LoggerDomain::Target);
    static G_LOG_DOMAIN: &str = "barbaz";
    init_test_logger();
    TEST_LOGGER.set_wrapped_logger(&THIS_LOGGER);

    let trace = collect_log(|| {
        warn!("foobar");
    });
    assert_eq!(trace.domain, Some("barbaz".to_string()));
    assert_eq!(trace.message, Some("foobar".to_string()));
    assert_eq!(trace.level, Some(glib_sys::G_LOG_LEVEL_WARNING));

    TEST_LOGGER.clear_wrapped_logger();
}