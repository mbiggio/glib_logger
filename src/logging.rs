use log::{Level, Metadata, Record};
use crate::{ Logger, LoggerDomain, LoggerType };

trait ToGlib {
    type GlibType;
    fn to_glib(&self) -> Self::GlibType;
}

impl ToGlib for log::Level {
    type GlibType = glib_sys::GLogLevelFlags;

    fn to_glib(&self) -> glib_sys::GLogLevelFlags {
        match *self {
            Level::Debug => glib_sys::G_LOG_LEVEL_DEBUG,
            Level::Info => glib_sys::G_LOG_LEVEL_INFO,
            Level::Warn => glib_sys::G_LOG_LEVEL_WARNING,
            Level::Trace => glib_sys::G_LOG_LEVEL_DEBUG,
            // cannot use G_LOG_LEVEL_ERROR as those are always fatal
            Level::Error => glib_sys::G_LOG_LEVEL_CRITICAL,
        }
    }
}

fn glib_log_structured(domain: Option<&str>, level: log::Level, file: &str, line: &str, func: &str, message: &str) {
    use glib_sys::g_log_structured_standard;
    use std::ffi::CString;
    use std::ptr;

    let c_file = CString::new(file).expect("CString::new(file) failed");
    let c_line = CString::new(line).expect("CString::new(line) failed");
    let c_func = CString::new(func).expect("CString::new(func) failed");
    let c_message = CString::new(message).expect("CString::new(message) failed");

    let c_domain_ptr = match domain {
        None => ptr::null(),
        Some(s) => match CString::new(s) {
            Ok(s) => s.as_ptr(),
            Err(_) => ptr::null(),
        },
    };

    unsafe {
        g_log_structured_standard(
            c_domain_ptr,
            level.to_glib(),
            c_file.as_ptr(),
            c_line.as_ptr(),
            c_func.as_ptr(),
            c_message.as_ptr(),
        );
    }
}

pub fn glib_log(domain: Option<&str>, level: log::Level, message: &str) {
    use glib_sys::g_log;
    use std::ffi::CString;
    use std::ptr;
    let c_message = CString::new(message).expect("CString::new(message) failed");

    let c_domain = match domain {
        None => None,
        Some(s) => Some(CString::new(s).expect("CString::new(domain) failed")),
    };

    let c_domain_ptr = match &c_domain {
        None => ptr::null(),
        Some(s) => s.as_ptr(),
    };

    unsafe {
        g_log(c_domain_ptr, level.to_glib(), c_message.as_ptr());
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let file = record.file().expect("no file in record");
        let line = &record.line().expect("no line in record").to_string();

        let domain = match self.domain {
            LoggerDomain::None => None,
            LoggerDomain::Custom(s) => Some(s),
            LoggerDomain::Target => Some(record.metadata().target()),
        };

        match self.logger_type {
            LoggerType::Simple => {
                let s = format!("{}:{}: {}", file, line, record.args());
                glib_log(domain, record.level(), &s);
            },
            LoggerType::SimplePlain => {
                let s = format!("{}", record.args());
                glib_log(domain, record.level(), &s)
            },
            LoggerType::Structured => {
                let s = format!("{}", record.args());
                glib_log_structured(
                    domain,
                    record.level(),
                    file,
                    line,
                    record.module_path().expect("no module"),
                    &s,
                );
            },
        };
    }

    fn flush(&self) {}
}

pub fn try_init(logger: &'static crate::Logger) -> Result<(), log::SetLoggerError> {
    log::set_logger(logger)
}
