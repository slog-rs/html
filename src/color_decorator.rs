use std::io;

use slog::Record;
use slog::Level;
use slog_stream::{Decorator, RecordDecorator};

fn level_to_color(lvl: Level) -> &'static str {
    match lvl {
        Level::Critical => "ff0000",
        Level::Error => "ff5500",
        Level::Warning => "ffaa00",
        Level::Info => "55aa00",
        Level::Debug => "aaaa7f",
        Level::Trace => "55557f",
    }
}

/// Record decorator (color)
pub struct ColorDecorator {
    msg_bold: bool,
}

impl ColorDecorator {
    /// New decorator that does color records
    pub fn new(msg_bold: bool) -> Self {
        ColorDecorator {
            msg_bold: msg_bold,
        }
    }
}

impl Decorator for ColorDecorator {
    type RecordDecorator = ColorRecordDecorator;

    fn decorate(&self, record: &Record) -> ColorRecordDecorator {
        ColorRecordDecorator {
            level_color: level_to_color(record.level()),
            msg_bold: self.msg_bold,
            key_bold: true,
        }
    }
}

/// Particular record decorator (color)
pub struct ColorRecordDecorator {
    level_color: &'static str,
    msg_bold: bool,
    key_bold: bool,
}

/// Methods: fmt_msg, fmt_key, fmt_separator, fmt_value, fmt_timestamp, fmt_level
impl RecordDecorator for ColorRecordDecorator {
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
        if self.msg_bold {
            try!(write!(io, "<span style=\"font-weight:bold\">"));
            try!(f(io));
            try!(write!(io, "</span>"));
        } else {
            try!(f(io));
        }
        Ok(())
    }

    fn fmt_key(&self,
               io: &mut io::Write,
               f: &Fn(&mut io::Write) -> io::Result<()>)
               -> io::Result<()> {
        if self.key_bold {
            try!(write!(io, "<span style=\"font-weight:bold;color:#55557f\">"));
            try!(f(io));
            try!(write!(io, "</span>"));
        } else {
            try!(f(io));
        }
        Ok(())
    }
}
