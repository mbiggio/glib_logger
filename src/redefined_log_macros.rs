#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => (
        log::log!(target: $target, log::Level::Error, $($arg)+);
    );
    ($($arg:tt)+) => (
        log::log!(target: G_LOG_DOMAIN, log::Level::Error, $($arg)+);
    )
}

#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => (
        log::log!(target: $target, log::Level::Warn, $($arg)+);
    );
    ($($arg:tt)+) => (
        log::log!(target: G_LOG_DOMAIN, log::Level::Warn, $($arg)+);
    )
}

#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => (
        log::log!(target: $target, log::Level::Info, $($arg)+);
    );
    ($($arg:tt)+) => (
        log::log!(target: G_LOG_DOMAIN, log::Level::Info, $($arg)+);
    )
}

#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => (
        log::log!(target: $target, log::Level::Debug, $($arg)+);
    );
    ($($arg:tt)+) => (
        log::log!(target: G_LOG_DOMAIN, log::Level::Debug, $($arg)+);
    )
}

#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => (
        log::log!(target: $target, log::Level::Trace, $($arg)+);
    );
    ($($arg:tt)+) => (
        log::log!(target: G_LOG_DOMAIN, log::Level::Trace, $($arg)+);
    )
}
