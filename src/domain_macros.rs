/// A macro which behaves exactly as `log::error!` except that it sets the
/// current log target to the contents of a `G_LOG_DOMAIN` library (and fails
/// to build if not defined).
#[macro_export]
macro_rules! g_error {
    (target: $target:expr, $($arg:tt)+) => (
        log::log!(target: $target, log::Level::Error, $($arg)+);
    );
    ($($arg:tt)+) => (
        log::log!(target: G_LOG_DOMAIN, log::Level::Error, $($arg)+);
    )
}

/// A macro which behaves exactly as `log::warn!` except that it sets the
/// current log target to the contents of a `G_LOG_DOMAIN` library (and fails
/// to build if not defined).
#[macro_export]
macro_rules! g_warn {
    (target: $target:expr, $($arg:tt)+) => (
        log::log!(target: $target, log::Level::Warn, $($arg)+);
    );
    ($($arg:tt)+) => (
        log::log!(target: G_LOG_DOMAIN, log::Level::Warn, $($arg)+);
    )
}

/// A macro which behaves exactly as `log::info!` except that it sets the
/// current log target to the contents of a `G_LOG_DOMAIN` library (and fails
/// to build if not defined).
#[macro_export]
macro_rules! g_info {
    (target: $target:expr, $($arg:tt)+) => (
        log::log!(target: $target, log::Level::Info, $($arg)+);
    );
    ($($arg:tt)+) => (
        log::log!(target: G_LOG_DOMAIN, log::Level::Info, $($arg)+);
    )
}

/// A macro which behaves exactly as `log::debug!` except that it sets the
/// current log target to the contents of a `G_LOG_DOMAIN` library (and fails
/// to build if not defined).
#[macro_export]
macro_rules! g_debug {
    (target: $target:expr, $($arg:tt)+) => (
        log::log!(target: $target, log::Level::Debug, $($arg)+);
    );
    ($($arg:tt)+) => (
        log::log!(target: G_LOG_DOMAIN, log::Level::Debug, $($arg)+);
    )
}

/// A macro which behaves exactly as `log::trace!` except that it sets the
/// current log target to the contents of a `G_LOG_DOMAIN` library (and fails
/// to build if not defined).
#[macro_export]
macro_rules! g_trace {
    (target: $target:expr, $($arg:tt)+) => (
        log::log!(target: $target, log::Level::Trace, $($arg)+);
    );
    ($($arg:tt)+) => (
        log::log!(target: G_LOG_DOMAIN, log::Level::Trace, $($arg)+);
    )
}
