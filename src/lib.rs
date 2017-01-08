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

mod decorator;
mod serializer;
mod color_palette;
mod style;

use std::io;
use std::sync::Mutex;

use slog::Record;
use slog::OwnedKeyValueList;
use slog_stream::{Decorator, RecordDecorator};

use decorator::HtmlDecorator;
use serializer::Serializer;
use style::StyleTable;
pub use style::Style;
pub use color_palette::ColorPalette;

/// Formatting mode
pub enum FormatMode {
    /// Compact logging format
    Compact,
    /// Full logging format
    Full,
}

pub enum Element {
    Timestamp,
    Message,
    Key,
    Value,
    Separator,
}

/// Html formatter
pub struct Format<D: Decorator> {
    mode: FormatMode,
    value_stack: Mutex<Vec<Vec<u8>>>,
    decorator: D,
    fn_timestamp: Box<TimestampFn>,
}

impl<D: Decorator> Format<D> {
    /// Create a new Html formatter
    pub fn new(mode: FormatMode, decorator: D, fn_timestamp: Box<TimestampFn>) -> Self {
        Format {
            mode: mode,
            value_stack: Mutex::new(Vec::new()),
            decorator: decorator,
            fn_timestamp: fn_timestamp,
        }
    }

    fn format_full(&self,
                   io: &mut io::Write,
                   record: &Record,
                   logger_values: &OwnedKeyValueList)
                   -> io::Result<()> {

        let r_decorator = self.decorator.decorate(record);

        try!(io.write_all(b"<pre style=\"margin-bottom:-0.5em\">"));

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

    fn format_compact(&self,
                      io: &mut io::Write,
                      record: &Record,
                      logger_values: &OwnedKeyValueList)
                      -> io::Result<()> {

        let mut value_stack = self.value_stack.lock().expect("failed to lock value_stack");
        let mut record_value_stack = try!(self.record_value_stack(record, logger_values));
        record_value_stack.reverse();
        let indent = record_value_stack.len();

        let mut changed = false;
        for i in 0..record_value_stack.len() {
            if value_stack.len() <= i || value_stack[i] != record_value_stack[i] {
                changed = true;
            }

            if changed {
                try!(io.write_all(b"<pre style=\"margin-bottom:-0.5em\">"));
                try!(self.print_indent(io, i));
                try!(io.write_all(&record_value_stack[i]));
                try!(io.write_all(b"</pre>\n"));
            }
        }
        if changed {
            *value_stack = record_value_stack;
        }

        let r_decorator = self.decorator.decorate(record);

        try!(io.write_all(b"<pre style=\"margin-bottom:-0.5em\">"));

        try!(self.print_indent(io, indent));
        try!(r_decorator.fmt_timestamp(io, &*self.fn_timestamp));
        try!(r_decorator.fmt_level(io, &|io: &mut io::Write| write!(io, " {} ", record.level().as_short_str())));
        try!(r_decorator.fmt_msg(io, &|io| write!(io, "{}", record.msg())));

        let mut serializer = Serializer::new(io, r_decorator);

        for &(k, v) in record.values().iter() {
            try!(serializer.print_comma());
            try!(v.serialize(record, k, &mut serializer));
        }

        let (mut io, _) = serializer.finish();

        io.write_all(b"</pre>\n")
    }

    /// Get formatted record_value_stack from `logger_values_ref`
    fn record_value_stack(&self,
                          record: &slog::Record,
                          logger_values_ref: &slog::OwnedKeyValueList)
                          -> io::Result<Vec<Vec<u8>>> {

        let mut value_stack = if let Some(logger_values) = logger_values_ref.values() {
            let r_decorator = self.decorator.decorate(record);
            let buf: Vec<u8> = Vec::with_capacity(128);
            let mut serializer = Serializer::new(buf, r_decorator);

            let mut clean = true;
            let mut logger_values = logger_values;
            loop {
                let (k, v) = logger_values.head();
                if !clean {
                    try!(serializer.print_comma());
                }
                try!(v.serialize(record, k, &mut serializer));
                clean = false;
                logger_values = if let Some(v) = logger_values.tail() {
                    v
                } else {
                    break;
                }
            }
            let (buf, _) = serializer.finish();
            vec![buf]
        } else {
            Vec::new()
        };

        if let &Some(ref parent) = logger_values_ref.parent() {
            let mut value = try!(self.record_value_stack(record, parent));
            value_stack.append(&mut value);
        }

        Ok(value_stack)
    }

    fn print_indent(&self, io: &mut io::Write, indent: usize) -> io::Result<()> {
        for _ in 0..indent {
            try!(write!(io, "  "));
        }
        Ok(())
    }
}

impl<D: Decorator> slog_stream::Format for Format<D> {
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        match self.mode {
            FormatMode::Compact => self.format_compact(io, record, logger_values),
            FormatMode::Full => self.format_full(io, record, logger_values),
        }
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
    mode: FormatMode,
    color_palette: ColorPalette,
    style: StyleTable,
    fn_timestamp: Box<TimestampFn>,
}

impl FormatBuilder {
    /// New `FormatBuilder`
    fn new() -> Self {
        FormatBuilder {
            mode: FormatMode::Full,
            color_palette: ColorPalette::default(),
            style: StyleTable::default(),
            fn_timestamp: Box::new(timestamp_local),
        }
    }

    /// Output using full mode (default)
    pub fn full(mut self) -> Self {
        self.mode = FormatMode::Full;
        self
    }

    /// Output using compact mode
    pub fn compact(mut self) -> Self {
        self.mode = FormatMode::Compact;
        self
    }

    /// Use custom color palette
    pub fn color_palette(mut self, color_palette: ColorPalette) -> Self {
        self.color_palette = color_palette;
        self
    }

    /// Use custom style for specified element
    pub fn style(mut self, element: Element, style: Style) -> Self {
        use Element::*;
        match element {
            Timestamp => self.style.timestamp = style,
            Message => self.style.message = style,
            Key => self.style.key = style,
            Value => self.style.value = style,
            Separator => self.style.separator = style,
        }
        self
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
    pub fn build(self) -> Format<HtmlDecorator> {
        Format {
            mode: self.mode,
            value_stack: Mutex::new(Vec::new()),
            decorator: HtmlDecorator::new(self.color_palette, self.style),
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
pub fn default() -> Format<HtmlDecorator> {
    FormatBuilder::new().build()
}
