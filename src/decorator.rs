use color_palette::ColorPalette;

use std::io;

use slog::Record;
use slog_stream::{Decorator, RecordDecorator};

/// Html decorator
pub struct HtmlDecorator {
    color_palette: ColorPalette,
    msg_bold: bool,
}

impl HtmlDecorator {
    pub fn new(color_palette: ColorPalette, msg_bold: bool) -> Self {
        HtmlDecorator {
            color_palette: color_palette,
            msg_bold: msg_bold,
        }
    }
}

impl Decorator for HtmlDecorator {
    type RecordDecorator = HtmlRecordDecorator;

    fn decorate(&self, record: &Record) -> HtmlRecordDecorator {
        HtmlRecordDecorator {
            level_color: self.color_palette.level_to_color(record.level()),
            msg_bold: self.msg_bold,
            key_bold: true,
        }
    }
}

/// Decorator for a particular record
pub struct HtmlRecordDecorator {
    level_color: &'static str,
    msg_bold: bool,
    key_bold: bool,
}

/// Methods: fmt_msg, fmt_key, fmt_separator, fmt_value, fmt_timestamp, fmt_level
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
