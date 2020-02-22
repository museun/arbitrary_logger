use crate::format::time::FormatTime;
use std::io::Write;

use super::color::RecordColorConfig;

#[cfg(feature = "color")]
use termcolor::{ColorSpec, WriteColor};

/// A record writer
pub struct Writer<'a, 'b: 'a> {
    #[allow(dead_code)]
    record_colors: RecordColorConfig,
    record: &'a log::Record<'b>,
}

/// Wrap a record with a writer
pub fn new_writer<'a, 'b: 'a>(
    record: &'a log::Record<'b>,
    record_colors: impl Into<Option<RecordColorConfig>>,
) -> Writer<'a, 'b> {
    Writer {
        record_colors: record_colors.into().unwrap_or_default(),
        record,
    }
}

impl<'a, 'b: 'a> Writer<'a, 'b> {
    #[inline(always)]
    fn inner_level<W: ?Sized + Write>(&self, buffer: &mut W) -> std::io::Result<()> {
        write!(buffer, "{:<5}", self.record.level())
    }

    #[inline(always)]
    fn inner_target<W: ?Sized + Write>(&self, buffer: &mut W) -> std::io::Result<()> {
        write!(buffer, "{}", self.record.target())
    }

    #[inline(always)]
    fn inner_timestamp<W: ?Sized + Write, T: ?Sized + FormatTime>(
        &self,
        mut buffer: &mut W,
        time: &T,
    ) -> std::io::Result<()> {
        write!(buffer, " ")?;
        time.format_time(&mut buffer)
    }

    #[inline(always)]
    fn inner_continuation<W: ?Sized + Write>(
        &self,
        buffer: &mut W,
        cont: &str,
    ) -> std::io::Result<()> {
        write!(buffer, "{}", cont)
    }

    #[inline(always)]
    fn inner_message<W: ?Sized + Write>(&self, buffer: &mut W) -> std::io::Result<()> {
        write!(buffer, " {}", self.record.args())
    }
}

impl<'a, 'b: 'a> Writer<'a, 'b> {
    /// Write the level
    #[cfg(not(feature = "color"))]
    pub fn level<W: ?Sized + Write>(&self, buffer: &mut W) -> std::io::Result<()> {
        self.inner_level(buffer)
    }

    /// Write the level
    #[cfg(feature = "color")]
    pub fn level<W: ?Sized + Write + WriteColor>(&self, buffer: &mut W) -> std::io::Result<()> {
        let color = match self.record.level() {
            log::Level::Error => self.record_colors.level.error,
            log::Level::Warn => self.record_colors.level.warn,
            log::Level::Info => self.record_colors.level.info,
            log::Level::Debug => self.record_colors.level.debug,
            log::Level::Trace => self.record_colors.level.trace,
        };

        buffer.set_color(ColorSpec::new().set_fg(Some(color)))?;
        self.inner_level(buffer)?;
        buffer.reset()
    }

    /// Write the target
    #[cfg(not(feature = "color"))]
    pub fn target<W: ?Sized + Write>(&self, buffer: &mut W) -> std::io::Result<()> {
        write!(buffer, " [")?;
        self.inner_target(buffer)?;
        write!(buffer, "]")
    }

    /// Write the target
    #[cfg(feature = "color")]
    pub fn target<W: ?Sized + Write + WriteColor>(&self, buffer: &mut W) -> std::io::Result<()> {
        write!(buffer, " [")?;
        buffer.set_color(ColorSpec::new().set_fg(self.record_colors.target.into()))?;
        self.inner_target(buffer)?;
        buffer.reset()?;
        write!(buffer, "]")
    }

    // Write the provided timestamp
    #[cfg(not(feature = "color"))]
    pub fn timestamp<W: ?Sized + Write, T: ?Sized + FormatTime>(
        &self,
        buffer: &mut W,
        time: &T,
    ) -> std::io::Result<()> {
        self.inner_timestamp(buffer, time)
    }

    // Write the provided timestamp
    #[cfg(feature = "color")]
    pub fn timestamp<W: ?Sized + Write + WriteColor, T: ?Sized + FormatTime>(
        &self,
        buffer: &mut W,
        time: &T,
    ) -> std::io::Result<()> {
        write!(buffer, " ")?;
        buffer.set_color(ColorSpec::new().set_fg(self.record_colors.timestamp.into()))?;
        self.inner_timestamp(buffer, time)?;
        buffer.reset()
    }

    /// Write a continuation (and insert a new line)
    #[cfg(not(feature = "color"))]
    pub fn continuation<W: ?Sized + Write>(
        &self,
        buffer: &mut W,
        cont: &str,
    ) -> std::io::Result<()> {
        writeln!(buffer)?;
        self.inner_continuation(buffer, cont)
    }

    /// Write a continuation (and insert a new line)
    #[cfg(feature = "color")]
    pub fn continuation<W: ?Sized + Write + WriteColor>(
        &self,
        buffer: &mut W,
        cont: &str,
    ) -> std::io::Result<()> {
        writeln!(buffer)?;
        buffer.set_color(ColorSpec::new().set_fg(self.record_colors.continuation.into()))?;
        self.inner_continuation(buffer, cont)?;
        buffer.reset()
    }

    /// Write the message
    #[cfg(not(feature = "color"))]
    pub fn message<W: ?Sized + Write>(&self, buffer: &mut W) -> std::io::Result<()> {
        self.inner_message(buffer)?;
        writeln!(buffer)
    }

    /// Write the message
    #[cfg(feature = "color")]
    pub fn message<W: ?Sized + Write + WriteColor>(&self, buffer: &mut W) -> std::io::Result<()> {
        buffer.set_color(ColorSpec::new().set_fg(self.record_colors.message.into()))?;
        self.inner_message(buffer)?;
        buffer.reset()?;
        writeln!(buffer)
    }
}
