/*! Formatting time types

*/

/// Primary trait for formatting time
pub trait FormatTime: Send + Sync {
    /// Format this time to the provided writer
    fn format_time(&self, w: &mut dyn std::io::Write) -> std::io::Result<()>;
}

/// Implementation for when no time should be formatted
impl FormatTime for () {
    fn format_time(&self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
        Ok(())
    }
}

/// Implementation for functions that takes a Writer
impl<'a, F> FormatTime for F
where
    F: Fn(&mut dyn std::io::Write) -> std::io::Result<()>,
    F: Send + Sync,
{
    fn format_time(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        (*self)(w)
    }
}

/// A Timestamp formatter
///
/// This is a UNIX-timestamp
#[derive(Default)]
pub struct Timestamp {
    style: TimestampStyle,
}

impl Timestamp {
    /// Create a new timestamp with the provided style
    pub fn new(style: TimestampStyle) -> Self {
        Self { style }
    }
}

impl FormatTime for Timestamp {
    fn format_time(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        let elapsed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

        let nanos = elapsed.subsec_nanos();
        match self.style {
            TimestampStyle::Whole => write!(w, "{}", elapsed.as_secs()),
            TimestampStyle::Fractional(width) if width == 0 => write!(w, "{}", elapsed.as_secs()),
            TimestampStyle::Fractional(width) => {
                write!(w, "{}.{}", elapsed.as_secs(), scale(nanos, width),)
            }
        }
    }
}

// TODO UTC timestamp

/// A running epoch counter
///
/// This prints time starting from a specific `Instant` (e.g. start of program)
pub struct Uptime {
    epoch: std::time::Instant,
    style: TimestampStyle,
}

impl Uptime {
    /// Create an Uptime starting at `now`
    pub fn now(style: TimestampStyle) -> Self {
        Self::new(std::time::Instant::now(), style)
    }

    /// Create an Uptime starting at a specific point in time
    pub fn new(epoch: std::time::Instant, style: TimestampStyle) -> Self {
        Self { epoch, style }
    }
}

impl FormatTime for Uptime {
    fn format_time(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        let elapsed = self.epoch.elapsed();
        match self.style {
            TimestampStyle::Whole => write!(w, "{}s", elapsed.as_secs()),
            TimestampStyle::Fractional(d) => write!(
                w,
                "{}.{}s",
                elapsed.as_secs(),
                scale(elapsed.subsec_nanos(), d)
            ),
        }
    }
}

impl From<std::time::Instant> for Uptime {
    fn from(epoch: std::time::Instant) -> Self {
        Self {
            epoch,
            style: TimestampStyle::Fractional(9),
        }
    }
}

/// Defaults to `now` and using **9** digits of fractional formatting
impl Default for Uptime {
    fn default() -> Self {
        Self {
            epoch: std::time::Instant::now(),
            style: TimestampStyle::Fractional(9),
        }
    }
}

/// Timestamp style to determine how timestamps should be printed
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum TimestampStyle {
    /// Just the whole seconds
    Whole,
    /// Include fractional time, up to `n` digits
    Fractional(usize),
}

/// Defaults to `Whole`
impl Default for TimestampStyle {
    fn default() -> Self {
        Self::Whole
    }
}

#[inline]
fn scale(d: u32, s: usize) -> u32 {
    if s > 9 {
        return d;
    }
    d / 10_usize.pow(9_usize.saturating_sub(s) as u32) as u32
}
