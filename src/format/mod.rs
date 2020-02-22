/*! Formating types

*/

mod writer;
pub use writer::*;

mod time;
pub use time::*;

mod color;
pub use color::*;

/// Primary trait for printing a log record
pub trait Print: Send + Sync {
    /// Print this log record
    fn print(&self, record: &log::Record) -> std::io::Result<()>;
}
