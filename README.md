A simple logger that integrates with [glib message
logging](https://developer.gnome.org/glib/unstable/glib-Message-Logging.html)
mechanism. The logger is useful when one wants to integrate a piece of Rust
code into a larger application which is already using the glib/gio stack, or
for Rust applications and libraries using gtk-rs or similar infrastructure.

### Simplest example

```rust
use std::env;

#[macro_use]
use log;

fn main() {
    env::set_var("G_MESSAGES_DEBUG", "all");

    glib_logger::init(&glib_logger::SIMPLE);
    log::set_max_level(log::LevelFilter::Debug);

    info!("info message: {}", 2);
    warn!("warning message: {}", "foobar");
    debug!("Hello, world!");
}
```

Equivalent Vala code:

```vala
public void main() {
    Environment.set_variable ("G_MESSAGES_DEBUG", "all", false);

    info("info message: %d", 2);
    warning("warning message: %s", "foobar");
    debug("Hello, world!");
}
```

Running:

```bash
$ ./glib_logger_test
** INFO: 20:18:34.074: src/main.rs:12: info message: 2

** (process:39403): WARNING **: 20:18:34.076: src/main.rs:13: warning message: foobar
** (process:39403): DEBUG: 20:18:34.076: src/main.rs:15: Hello, world!
```

### Examples

The crate provides a series of examples in the `examples/` subdir.

- simple.rs : The simplest usage example possible, zero customization.
- simple_domain.rs : How to use a simple, custom glib log domain.
- target_domain.rs : How to map the Rust log target to the glib log domain.
- g_macro_domain.rs : How to use macros to have the glib log domain specified
    in `G_LOG_DOMAIN`.
- override_macro_domain.rs : How to use the optional feature to remap the
    g_trace etc. macros over the standard log macro names
- custom_log_function.rs : How to specify a custom log function using
    the `glib_sys` crate.


### Logger types

The crate makes two logger types available, plus the functionality to build a
custom logger.

The two predefined logger types are:

- `glib_logger::SIMPLE`: a simple logger which prints the message, decorated
    with file and line number, without a domain

- `glib_logger::STRUCTURED`: an experimental logger using glib structured
    logging capabilities

Custom loggers can be defined with `glib_logger::custom`, specifying both how
the message is composed and how the logging domain is composed.

`LoggerType` can be:

- `LoggerType::Simple`: a simple logger which prints the message, decorated
    with file and line number

- `LoggerType::SimplePlain`: a simple logger which prints the message, without
    decorating the message with file and line number

- `LoggerType::Structured`: an experimental logger using glib structured
    logging capabilities

`LoggerDomain` can be:

- `LoggerDomain::None`: a logger which uses an empty domain

- `LoggerDomain::Custom(&'static str)`: a logger using a predefined domain; note
    that the domain would then be the same across all Rust crates

- `LoggerDomain::Context`: a logger using the logging context of the `log` crate
    as the glib logging domain

See the domain section for further details.

### Log levels

Due to slight differences between the meaning of respective log levels, the
crate takes certain liberties. Specifically the log level mappings are:

- `Level::Trace`, `Level::Debug` => G_LOG_LEVEL_DEBUG
- `Level::Error` => G_LOG_LEVEL_CRITICAL
- `Level::Info` => G_LOG_LEVEL_INFO
- `Level::Warn` => G_LOG_LEVEL_WARNING

The G_LOG_LEVEL_ERROR (as produced via `g_error()` macro in C) is not mapped to
any of `log::Level` enum values. The reason is that `g_error()` is fatal, while
`log::error!()` is not.

The formatting is done fully in Rust. Log filtering based on level is done in
both Glib and the Rust `log` crate.

It is advisable to set `G_MESSAGES_DEBUG=all` environment variable if a custom
glib log handler is not used, to set the glib logger to debug level.

Additionally log level will be filtered also by the `log` crate in Rust itself;
so to enable lower level of logs you might need to set the log level explicitely
using calls similar to `log::set_max_level(log::LevelFilter::Debug);`.

### Domain

Using Glib a domain can be set per file by using `#define G_LOG_DOMAIN
"my-domain"` directly in C code.

This functionality is not available by default when using the predefined
`glib_logger` loggers, so a custom logger must be created using the `custom`
function.

The closest option to get this functionality is using a custom logger with
LoggerDomain set as `LoggerDomain::Target`.

This example shows how to use the target to set the domain using standard Rust
log functions:

```rust
// initialize a static custom logger
static CUSTOM: Logger = custom(LoggerType::Simple, LoggerDomain::Target);
// set the logger as active
glib_logger::init(&CUSTOM);
// implicit; will use the current crate/file as domain
warn!("some log message");
// explicit; will use the "my-domain" string as a domain
warn!(target: "my-domain", "some log message");
```

Alternatively, you can use macros and `G_LOG_DOMAIN` in the same vein of glib
to support the domain functionality:

```rust
// See the "g_macro_domain" example for a more extensive usage example.
#[macro_use]
extern crate glib_logger;
static G_LOG_DOMAIN: &str = "my-global-domain";
static CUSTOM: Logger = custom(LoggerType::Simple, LoggerDomain::Target);
// ...
// set the logger as active
glib_logger::init(&CUSTOM);
// implicit; will use the domain in G_LOG_DOMAIN
g_warn!("some log message");
// explicit; will use the "my-domain" string as a domain
g_warn!(target: "my-domain", "some log message");
```

Finally, you can use macros in the same vein of glib ones to support the
domain functionality:

```rust
// This requires the "redefine_log_macros" to be enabled in Cargo.toml.
// See the "override_macro_domain" example for a more extensive usage example.
#[macro_use]
extern crate glib_logger;
static G_LOG_DOMAIN: &str = "my-global-domain";
static CUSTOM: Logger = custom(LoggerType::Simple, LoggerDomain::Target);
// ...
// set the logger as active
glib_logger::init(&CUSTOM);
// implicit; will use the domain in G_LOG_DOMAIN
warn!("some log message");
// explicit; will use the "my-domain" string as a domain
warn!(target: "my-domain", "some log message");
```
