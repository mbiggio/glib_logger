A simple logger that integrates with [glib message
logging][https://developer.gnome.org/glib/unstable/glib-Message-Logging.html]
mechanism. The logger is useful when one wants to integrate a piece of Rust code
into a larger application which is already using glib/gio stack.

### Example

```rust
use std::env;

use log;

fn main() {
    env::set_var("G_MESSAGES_DEBUG", "all");

    glib_logger::init(&glib_logger::SIMPLE);
    log::set_max_level(log::LevelFilter::Debug);

    log::info!("info message: {}", 2);
    log::warn!("warning message: {}", "foobar");
    log::debug!("Hello, world!");
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

### Details

Due to slight differences between the meaning of respective log levels, the
crate takes certain liberties. Specifically the log level mappings are:

- `Level::Trace`, `Level::Debug` => G_LOG_LEVEL_DEBUG
- `Level::Error` => G_LOG_LEVEL_CRITICAL
- `Level::Info` => G_LOG_LEVEL_INFO
- `Level::Warn` => G_LOG_LEVEL_WARNING

The G_LOG_LEVEL_ERROR (as produced via `g_error()` macro in C) is not mapped to
any of `log::Level` enum values. The reason is that `g_error()` is fatal, while
`log::error!()` is not.

The formatting is done fully in Rust. However, log filtering based on level is
done in Glib. It is advisable to set `G_MESSAGES_DEBUG=all` environment variable.

Using Glib a domain can be set per file by using `#define G_LOG_DOMAIN
"my-domain"` directly in C code. This functionality is not avaialble when using
`glib_logger`, all logs are emitted with a NULL domain.
