//! A pretty logger
//!
use crate::format::{
    self, FormatTime, Print, RecordColorConfig, Timestamp, TimestampStyle, Uptime,
};

/// A pretty logger
pub struct Pretty {
    continuation: Option<String>,
    time: Option<Box<dyn FormatTime>>,

    #[allow(dead_code)]
    use_color: bool,
    level: bool,
    target: bool,

    record_colors: RecordColorConfig,
}

impl Default for Pretty {
    fn default() -> Self {
        Self {
            continuation: None,
            time: None,

            use_color: true,
            level: true,
            target: true,

            record_colors: Default::default(),
        }
    }
}

impl Pretty {
    /// get a builder for the pretty logger
    pub fn builder() -> PrettyBuilder {
        PrettyBuilder::default()
    }
}

impl Print for Pretty {
    #[inline]
    fn print(&self, record: &log::Record) -> std::io::Result<()> {
        let writer = format::new_writer(record, self.record_colors);

        #[cfg(not(feature = "color"))]
        let out = std::io::stdout();
        #[cfg(not(feature = "color"))]
        let mut buffer = out.lock();

        #[cfg(feature = "color")]
        let buf_writer = termcolor::BufferWriter::stdout(if self.use_color {
            termcolor::ColorChoice::Auto
        } else {
            termcolor::ColorChoice::Never
        });
        #[cfg(feature = "color")]
        let mut buffer = buf_writer.buffer();

        if self.level {
            writer.level(&mut buffer)?;
        }
        if self.target {
            writer.target(&mut buffer)?;
        }
        if let Some(time) = self.time.as_deref() {
            writer.timestamp(&mut buffer, time)?;
        }
        if let Some(ref cont) = self.continuation {
            writer.continuation(&mut buffer, &cont)?;
        }
        writer.message(&mut buffer)?;

        #[cfg(feature = "color")]
        buf_writer.print(&buffer)?;

        Ok(())
    }
}

/// Configure a Pretty logger
pub struct PrettyBuilder {
    time: Option<Box<dyn FormatTime>>,
    target: bool,
    level: bool,
    use_color: bool,
    continuation: Option<String>,
    record_colors: RecordColorConfig,
}

impl Default for PrettyBuilder {
    fn default() -> Self {
        let (time, target, level, continuation, record_colors) = Default::default();
        Self {
            use_color: true,
            time,
            target,
            level,
            continuation,
            record_colors,
        }
    }
}

impl PrettyBuilder {
    #[cfg(feature = "color")]
    pub fn with_custom_colors(mut self, config: RecordColorConfig) -> Self {
        self.record_colors = config;
        self
    }

    #[cfg(feature = "color")]
    pub fn with_color(mut self) -> Self {
        self.use_color = true;
        self
    }

    #[cfg(feature = "color")]
    pub fn without_color(mut self) -> Self {
        self.use_color = false;
        self
    }

    pub fn with_time<F: FormatTime + 'static>(mut self, time: F) -> Self {
        self.time.replace(Box::new(time));
        self
    }

    pub fn without_time(mut self) -> Self {
        self.time.take();
        self
    }

    pub fn with_target(mut self) -> Self {
        self.target = true;
        self
    }

    pub fn without_target(mut self) -> Self {
        self.target = false;
        self
    }

    pub fn with_level(mut self) -> Self {
        self.level = true;
        self
    }

    pub fn without_level(mut self) -> Self {
        self.level = false;
        self
    }

    pub fn with_continuation<'a>(mut self, cont: impl Into<Option<&'a str>>) -> Self {
        self.continuation.replace(
            cont.into()
                .unwrap_or_else(|| crate::DEFAULT_CONTINUATION)
                .to_string(),
        );
        self
    }

    pub fn without_continuation(mut self) -> Self {
        self.continuation.take();
        self
    }

    pub fn uptime(self) -> Self {
        self.with_time(Uptime::default())
    }

    pub fn unix_timestamp(self, style: impl Into<Option<TimestampStyle>>) -> Self {
        self.with_time(Timestamp::new(style.into().unwrap_or_default()))
    }

    pub fn build(self) -> Pretty {
        Pretty {
            continuation: self.continuation,
            level: self.level,
            target: self.target,
            time: self.time,
            use_color: self.use_color,
            record_colors: self.record_colors,
        }
    }
}
