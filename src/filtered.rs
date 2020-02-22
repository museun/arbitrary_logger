//! Filtering types
//!
//! # Usage
//! ```rust
//! # use arbitrary_logger::filtered::Filtered;
//! let filtered = Filtered::from_str("foo::bar=trace,baz=debug");
//! # assert!(filtered.filtered().any(|(k, &v)| { k == "foo::bar" && v == log::LevelFilter::Trace }));
//! # assert!(filtered.filtered().any(|(k, &v)| { k == "baz" && v == log::LevelFilter::Debug }));
//! let filtered = Filtered::new(&["foo::bar=trace", "baz=debug"]);
//! # assert!(filtered.filtered().any(|(k, &v)| { k == "foo::bar" && v == log::LevelFilter::Trace }));
//! # assert!(filtered.filtered().any(|(k, &v)| { k == "baz" && v == log::LevelFilter::Debug }));
//!
//! std::env::set_var("RUST_LOG", "foo::bar=trace,baz=debug");
//! let filtered = Filtered::from_env();
//! # assert!(filtered.filtered().any(|(k, &v)| { k == "foo::bar" && v == log::LevelFilter::Trace }));
//! # assert!(filtered.filtered().any(|(k, &v)| { k == "baz" && v == log::LevelFilter::Debug }));
//!
//! std::env::set_var("MY_RUST_LOG", "foo::bar=trace,baz=debug");
//! let filtered = Filtered::from_env_key("MY_RUST_LOG");
//! # assert!(filtered.filtered().any(|(k, &v)| { k == "foo::bar" && v == log::LevelFilter::Trace }));
//! # assert!(filtered.filtered().any(|(k, &v)| { k == "baz" && v == log::LevelFilter::Debug }));
//! ```
use std::collections::HashMap;

/// A simple target-filtering type
#[derive(Default)]
pub struct Filtered {
    targets: HashMap<String, log::LevelFilter>,
}

impl Filtered {
    /// Create a new filtered set from an iterator of strings
    ///
    /// The format should be `target=level`
    pub fn new<I, S>(targets: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        Self {
            targets: targets
                .into_iter()
                .map(|s| s.to_string())
                .filter_map(|s| {
                    let mut iter = s.split('=');
                    (iter.next()?.to_string(), parse_level(iter.next()?)).into()
                })
                .collect(),
        }
    }

    /// Create a new filtered set from a string
    ///
    /// The format should be `target1=level,target2=level`
    pub fn from_str(string: impl AsRef<str>) -> Self {
        Self::new(string.as_ref().split(","))
    }

    /// Create a fitlered set from the environment, reading `RUST_LOG`
    pub fn from_env() -> Self {
        Self::from_env_key("RUST_LOG")
    }

    /// Create a fitlered set from the environment, reading from the provided key
    pub fn from_env_key(key: &str) -> Self {
        match std::env::var(key) {
            Ok(value) => Self::new(value.split(',')),
            _ => Self {
                targets: Default::default(),
            },
        }
    }

    /// Returns an iterator of the `target,level` pairs
    pub fn filtered(&self) -> impl Iterator<Item = (&String, &log::LevelFilter)> {
        self.targets.iter()
    }

    #[inline]
    pub(crate) fn apply(&self, input: &str, level: log::Level) -> bool {
        self.targets.iter().any(|(k, v)| {
            if !input.starts_with(k) || !input.contains("::") && k != input {
                return false;
            }
            level >= *v
        })
    }
}

#[inline]
fn parse_level(s: &str) -> log::LevelFilter {
    match s {
        s if s.eq_ignore_ascii_case("trace") => log::LevelFilter::Trace,
        s if s.eq_ignore_ascii_case("debug") => log::LevelFilter::Debug,
        s if s.eq_ignore_ascii_case("info") => log::LevelFilter::Info,
        s if s.eq_ignore_ascii_case("warn") => log::LevelFilter::Warn,
        s if s.eq_ignore_ascii_case("error") => log::LevelFilter::Error,
        _ => log::LevelFilter::Off,
    }
}
