// The MIT License (MIT)
//
// Copyright (c) 2020 Maciek Borzecki <maciek.borzecki@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! A simple logger that integrates with [glib message
//! logging][https://developer.gnome.org/glib/unstable/glib-Message-Logging.html]
//! mechanism. The logger is useful when one wants to integrate a piece of Rust
//! code into a larger application which is already using glib/gio stack.
//!
//! ### Example
//!
//! ```
//! use std::env;
//!
//! use log;
//!
//! fn main() {
//!     env::set_var("G_MESSAGES_DEBUG", "all");
//!
//!     glib_logger::init(&glib_logger::SIMPLE);
//!     log::set_max_level(log::LevelFilter::Debug);
//!
//!     log::info!("info message: {}", 2);
//!     log::warn!("warning message: {}", "foobar");
//!     log::debug!("Hello, world!");
//! }
//! ```
//!
//! Equivalent Vala code:

//! ```vala
//! public void main() {
//!     Environment.set_variable ("G_MESSAGES_DEBUG", "all", false);
//!
//!     info("info message: %d", 2);
//!     warning("warning message: %s", "foobar");
//!     debug("Hello, world!");
//! }
//! ```
//!
//! Running:

//! ```bash
//! $ ./glib_logger_test
//! ** INFO: 20:18:34.074: src/main.rs:12: info message: 2
//!
//! ** (process:39403): WARNING **: 20:18:34.076: src/main.rs:13: warning message: foobar
//! ** (process:39403): DEBUG: 20:18:34.076: src/main.rs:15: Hello, world!
//! ```
//!
//! ### Details
//!
//! Due to slight differences between the meaning of respective log levels, the
//! crate takes certain liberties. Specifically the log level mappings are:
//!
//! - `Level::Trace`, `Level::Debug` => G_LOG_LEVEL_DEBUG
//! - `Level::Error` => G_LOG_LEVEL_CRITICAL
//! - `Level::Info` => G_LOG_LEVEL_INFO
//! - `Level::Warn` => G_LOG_LEVEL_WARNING
//!
//! The G_LOG_LEVEL_ERROR (as produced via `g_error()` macro in C) is not mapped to
//! any of `log::Level` enum values. The reason is that `g_error()` is fatal, while
//! `log::error!()` is not.
//!
//! The formatting is done fully in Rust. However, log filtering based on level is
//! done in Glib. It is advisable to set `G_MESSAGES_DEBUG=all` environment variable.
//!
//! Using Glib a domain can be set per file by using `#define G_LOG_DOMAIN
//! "my-domain"` directly in C code. This functionality is not avaialble when using
//! `glib_logger`, all logs are emitted with a NULL domain.

use log::{Level, Metadata, Record};

pub struct Logger {
    structured: bool,
}

pub const fn simple() -> Logger {
    Logger { structured: false }
}

pub const fn structured() -> Logger {
    Logger { structured: true }
}

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

fn glib_log_structured(level: log::Level, file: &str, line: &str, func: &str, message: &str) {
    use glib_sys::g_log_structured_standard;
    use std::ffi::CString;
    use std::ptr;
    println!("file: {}", file);
    let c_file = CString::new(file).expect("CString::new(file) failed");
    let c_line = CString::new(line).expect("CString::new(line) failed");
    let c_func = CString::new(func).expect("CString::new(func) failed");
    let c_message = CString::new(message).expect("CString::new(message) failed");
    unsafe {
        g_log_structured_standard(
            ptr::null(),
            level.to_glib(),
            c_file.as_ptr(),
            c_line.as_ptr(),
            c_func.as_ptr(),
            c_message.as_ptr(),
        );
    }
}

fn glib_log(level: log::Level, message: &str) {
    use glib_sys::g_log;
    use std::ffi::CString;
    use std::ptr;
    let c_message = CString::new(message).expect("CString::new(message) failed");
    unsafe {
        g_log(ptr::null(), level.to_glib(), c_message.as_ptr());
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
        if !self.structured {
            let s = format!("{}:{}: {}", file, line, record.args());
            glib_log(record.level(), &s);
        } else {
            let s = format!("{}", record.args());
            glib_log_structured(
                record.level(),
                file,
                line,
                record.module_path().expect("no module"),
                &s,
            );
        }
    }

    fn flush(&self) {}
}

// Simple logger.
pub static SIMPLE: Logger = simple();

// Structured logger (Experimental).
pub static STRUCTURED: Logger = structured();

// Set up given logger.
pub fn init(logger: &'static Logger) {
    log::set_logger(logger).expect("glib_logger::init failed to initialize");
}

#[cfg(test)]
mod tests {
    use log::Level;
    use std::os::raw::c_char;

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

        // let mut trace = Box::new(LogTrace::new());
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

        println!("log writer");
    }

    #[test]
    fn simple_log() {
        let trace = collect_log(|| {
            super::glib_log(Level::Debug, "foobar");
        });
        assert_eq!(trace.domain, None);
        assert_eq!(trace.message, Some("foobar".to_string()));
        assert_eq!(trace.level, Some(glib_sys::G_LOG_LEVEL_DEBUG));
    }

    #[test]
    fn simple_formatted_log() {
        let trace = collect_log(|| {
            super::glib_log(
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
}
