use color_palette::ColorPalette;
use style::{Style, StyleTable};

use std::io;

use slog::Record;
use slog_stream::{Decorator, RecordDecorator};

/// Html decorator
pub struct HtmlDecorator {
    color_palette: ColorPalette,
    style: StyleTable,
}

impl HtmlDecorator {
    pub fn new(color_palette: ColorPalette, style: StyleTable) -> Self {
        HtmlDecorator {
            color_palette: color_palette,
            style: style,
        }
    }
}

impl Decorator for HtmlDecorator {
    type RecordDecorator = HtmlRecordDecorator;

    fn decorate(&self, record: &Record) -> HtmlRecordDecorator {
        HtmlRecordDecorator {
            level_color: self.color_palette.level_to_color(record.level()),
            style: self.style,
        }
    }
}

/// Decorator for a particular record
pub struct HtmlRecordDecorator {
    level_color: &'static str,
    style: StyleTable,
}

fn fmt(io: &mut io::Write,
       f: &Fn(&mut io::Write) -> io::Result<()>,
       style: &Style)
       -> io::Result<()> {
    if style.color.is_some() || style.bold {
        try!(write!(io, "<span style=\""));
        if let Some(color) = style.color {
            try!(write!(io, "color:#{};", color));
        }
        if style.bold {
            try!(write!(io, "font-weight:bold;"));
        }
        try!(write!(io, "\">"));
        try!(f(io));
        try!(write!(io, "</span>"));
    } else {
        try!(f(io));
    }
    Ok(())
}

impl RecordDecorator for HtmlRecordDecorator {
    fn fmt_level(&self,
                 io: &mut io::Write,
                 f: &Fn(&mut io::Write) -> io::Result<()>)
                 -> io::Result<()> {
        try!(write!(io, "<span style=\"color:#{}\">", self.level_color));
        try!(f(io));
        try!(write!(io, "</span>"));
        Ok(())
    }

    fn fmt_msg(&self,
               io: &mut io::Write,
               f: &Fn(&mut io::Write) -> io::Result<()>)
               -> io::Result<()> {
        fmt(io, f, &self.style.message)
    }

    fn fmt_key(&self,
               io: &mut io::Write,
               f: &Fn(&mut io::Write) -> io::Result<()>)
               -> io::Result<()> {
        fmt(io, f, &self.style.key)
    }

    fn fmt_separator(&self,
               io: &mut io::Write,
               f: &Fn(&mut io::Write) -> io::Result<()>)
               -> io::Result<()> {
        fmt(io, f, &self.style.separator)
    }

    fn fmt_value(&self,
               io: &mut io::Write,
               f: &Fn(&mut io::Write) -> io::Result<()>)
               -> io::Result<()> {
        fmt(io, f, &self.style.value)
    }

    fn fmt_timestamp(&self,
               io: &mut io::Write,
               f: &Fn(&mut io::Write) -> io::Result<()>)
               -> io::Result<()> {
        fmt(io, f, &self.style.timestamp)
    }
}
