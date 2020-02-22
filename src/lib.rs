use once_cell::sync::OnceCell;

static INSTANCE: OnceCell<Logger> = OnceCell::new();

#[doc(inline)]
pub mod filtered;

#[doc(inline)]
pub mod format;

#[doc(inline)]
pub mod logger;

struct Logger {
    fmt: Box<dyn format::Print>,
    filters: Option<filtered::Filtered>,
    min_level: log::LevelFilter,
}

impl Logger {
    #[inline]
    fn print(&self, record: &log::Record) {
        let _ = self.fmt.print(record);
    }
}

impl log::Log for Logger {
    #[inline(always)]
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.min_level
    }

    #[inline]
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            if let Some(filters) = &self.filters {
                if !filters.apply(record.target(), record.level()) {
                    self.print(record);
                }
            } else {
                self.print(record);
            }
        }
    }

    #[inline(always)]
    fn flush(&self) {}
}

/// Init a logger with a minimum level and no filters
pub fn init<F: format::Print + 'static>(
    format: F,
    min_level: log::LevelFilter,
) -> Result<(), log::SetLoggerError> {
    let instance = INSTANCE.get_or_init(|| Logger {
        fmt: Box::new(format),
        min_level,
        filters: None,
    });

    log::set_max_level(log::LevelFilter::Trace);
    log::set_logger(instance)
}

/// Init a logger with a minimum level and filters
pub fn init_with_filters<F>(
    format: F,
    min_level: log::LevelFilter,
    filters: filtered::Filtered,
) -> Result<(), log::SetLoggerError>
where
    F: format::Print + 'static,
{
    let instance = INSTANCE.get_or_init(|| Logger {
        fmt: Box::new(format),
        min_level,
        filters: filters.into(),
    });

    log::set_max_level(log::LevelFilter::Trace);
    log::set_logger(instance)
}

/// Init a logger with a minimum level and filters, ignoring any errors
pub fn try_init_with_filters<F>(format: F, min_level: log::LevelFilter, filters: filtered::Filtered)
where
    F: format::Print + 'static,
{
    let _ = init_with_filters(format, min_level, filters);
}

/// Init a logger with a minimum level and no filters, ignoring any errors
pub fn try_init<F: format::Print + 'static>(format: F, min_level: log::LevelFilter) {
    let _ = crate::init(format, min_level);
}

/// Different continuation string: `⤷`
pub static DEFAULT_CONTINUATION: &str = "⤷";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_filtered() {
        let filtered = filtered::Filtered::new(&["tokio::io=trace", "mio=debug"]);

        for (ignored, level) in &[
            ("mio::uds::spam", log::Level::Trace),
            ("tokio::io::Read", log::Level::Trace),
        ] {
            assert!(filtered.apply(ignored, *level), "{}: {:?}", ignored, level);
        }

        for (allowed, level) in &[
            ("tokio", log::Level::Info),
            ("mionext", log::Level::Trace),
            ("mio::foo", log::Level::Warn),
            ("tokio::sync::mutex", log::Level::Debug),
            ("tokio_util::io", log::Level::Warn),
        ] {
            assert!(!filtered.apply(allowed, *level), "{}: {:?}", allowed, level);
        }
    }

    #[test]
    fn new_rust_log() {
        std::env::set_var("RUST_LOG", "tokio=trace, ")
    }

    pub mod hidden {
        pub mod other {
            pub fn work() {
                log::debug!("work!");
            }
        }
        pub fn work() {
            log::debug!("work!");
        }
    }
    pub mod nested {
        pub mod hidden {
            pub fn work() {
                log::debug!("work!");
            }
        }
        pub fn work() {
            log::debug!("work!");
        }
    }

    #[test]
    fn log() {
        crate::try_init_with_filters(
            crate::logger::Pretty::default(),
            log::LevelFilter::Trace,
            filtered::Filtered::new(&[
                "arbitrary_logger::tests::hidden=warn",
                "arbitrary_logger::tests::nested::hidden=info",
            ]),
        );

        log::trace!("hello world");
        log::debug!("hello world");
        log::info!("hello world");
        log::warn!("hello world");
        log::error!("hello world");

        hidden::work();
        hidden::other::work();
        nested::hidden::work();
        nested::work();
    }
}
