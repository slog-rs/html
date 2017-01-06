//! Html formatter for `slog-rs`
//!
//! # Example
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_html;
//! extern crate slog_stream;
//!
//! use slog::DrainExt;
//!
//! use std::fs::OpenOptions;
//!
//! fn main() {
//!     let file = OpenOptions::new()
//!         .create(true)
//!         .write(true)
//!         .truncate(true)
//!         .open("target/log.html").unwrap();
//!
//!     let log = slog::Logger::root(
//!         slog_stream::stream(
//!             file,
//!             slog_html::default()
//!         ).fuse(),
//!         o!("version" => env!("CARGO_PKG_VERSION"))
//!     );
//!
//!     debug!(log, "debug values"; "x" => 1, "y" => -1);
//! }
//! ```
#![warn(missing_docs)]

#[macro_use]
extern crate slog;
extern crate slog_stream;
extern crate chrono;

mod color_decorator;
mod serializer;

use std::io;

use slog::Record;
use slog::OwnedKeyValueList;
use slog_stream::{Decorator, RecordDecorator};

use color_decorator::ColorDecorator;
use serializer::Serializer;

/// Html formatter
pub struct Format<D: Decorator> {
    decorator: D,
    fn_timestamp: Box<TimestampFn>,
}

impl<D: Decorator> Format<D> {
    /// Create a new Html formatter
    pub fn new(decorator: D, fn_timestamp: Box<TimestampFn>) -> Self {
        Format {
            decorator: decorator,
            fn_timestamp: fn_timestamp,
        }
    }
}

impl<D: Decorator> slog_stream::Format for Format<D> {
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        let r_decorator = self.decorator.decorate(record);

        try!(io.write_all(b"<pre>"));

        try!(r_decorator.fmt_timestamp(io, &*self.fn_timestamp));
        try!(r_decorator.fmt_level(io, &|io: &mut io::Write| write!(io, " {} ", record.level().as_short_str())));
        try!(r_decorator.fmt_msg(io, &|io| write!(io, "{}", record.msg())));

        let mut serializer = Serializer::new(io, r_decorator);

        for (k, v) in logger_values.iter() {
            try!(serializer.print_comma());
            try!(v.serialize(record, k, &mut serializer));
        }

        for &(k, v) in record.values().iter() {
            try!(serializer.print_comma());
            try!(v.serialize(record, k, &mut serializer));
        }

        let (mut io, _) = serializer.finish();

        io.write_all(b"</pre>\n")
    }
}

/// Timestamp function type
pub type TimestampFn = Fn(&mut io::Write) -> io::Result<()> + Send + Sync;

const TIMESTAMP_FORMAT: &'static str = "%b %d %H:%M:%S%.3f";

/// Default local timestamp function used by `Format`
///
/// The exact format used, is still subject to change.
pub fn timestamp_local(io: &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::Local::now().format(TIMESTAMP_FORMAT))
}

/// Default UTC timestamp function used by `Format`
///
/// The exact format used, is still subject to change.
pub fn timestamp_utc(io: &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::UTC::now().format(TIMESTAMP_FORMAT))
}

/// Streamer builder
pub struct FormatBuilder {
    fn_timestamp: Box<TimestampFn>,
}

impl FormatBuilder {
    /// New `FormatBuilder`
    fn new() -> Self {
        FormatBuilder {
            fn_timestamp: Box::new(timestamp_local),
        }
    }

    /// Use the UTC time zone for the timestamp
    pub fn use_utc_timestamp(mut self) -> Self {
        self.fn_timestamp = Box::new(timestamp_utc);
        self
    }

    /// Use the local time zone for the timestamp (default)
    pub fn use_local_timestamp(mut self) -> Self {
        self.fn_timestamp = Box::new(timestamp_local);
        self
    }

    /// Provide a custom function to generate the timestamp
    pub fn use_custom_timestamp<F>(mut self, f: F) -> Self
        where F: Fn(&mut io::Write) -> io::Result<()> + 'static + Send + Sync
    {
        self.fn_timestamp = Box::new(f);
        self
    }

    /// Build Html formatter
    pub fn build(self) -> Format<ColorDecorator> {
        Format {
            decorator: ColorDecorator::default(),
            fn_timestamp: self.fn_timestamp,
        }
    }
}

impl Default for FormatBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Create new `FormatBuilder` to create `Format`
pub fn new() -> FormatBuilder {
    FormatBuilder::new()
}

/// Default html `Format`
pub fn default() -> Format<ColorDecorator> {
    FormatBuilder::new().build()
}
