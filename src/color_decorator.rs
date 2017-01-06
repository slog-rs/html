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

/// Record decorator (color) for terminal output
pub struct ColorDecorator {
    use_color: bool,
}

impl ColorDecorator {
    /// New decorator that does color records
    pub fn new_colored() -> Self {
        ColorDecorator {
            use_color: true,
        }
    }

    /// New decorator that does not color records
    pub fn new_plain() -> Self {
        ColorDecorator {
            use_color: false,
        }
    }
}

impl Default for ColorDecorator {
    fn default() -> Self {
        ColorDecorator {
            use_color: true,
        }
    }
}

impl Decorator for ColorDecorator {
    type RecordDecorator = ColorRecordDecorator;

    fn decorate(&self, record: &Record) -> ColorRecordDecorator {
        if self.use_color {
            ColorRecordDecorator {
                level_color: Some(level_to_color(record.level())),
                key_bold: true,
            }
        } else {
            ColorRecordDecorator {
                level_color: None,
                key_bold: false,
            }
        }
    }
}

/// Particular record decorator (color) for terminal output
pub struct ColorRecordDecorator {
    level_color: Option<&'static str>,
    key_bold: bool,
}

/// Methods: fmt_msg, fmt_key, fmt_separator, fmt_value, fmt_timestamp, fmt_level
impl RecordDecorator for ColorRecordDecorator {
    fn fmt_level(&self,
                 io: &mut io::Write,
                 f: &Fn(&mut io::Write) -> io::Result<()>)
                 -> io::Result<()> {
        if let Some(level_color) = self.level_color {
            try!(write!(io, "<span style=\"color:#{}\">", level_color));
            try!(f(io));
            try!(write!(io, "</span>"));
        } else {
            try!(f(io));
        }
        Ok(())
    }

    fn fmt_msg(&self,
               io: &mut io::Write,
               f: &Fn(&mut io::Write) -> io::Result<()>)
               -> io::Result<()> {
        if self.key_bold {
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
