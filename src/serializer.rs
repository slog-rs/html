use std::{io, fmt};

use slog::ser;
use slog_stream::RecordDecorator;

pub struct Serializer<W, D: RecordDecorator> {
    io: W,
    decorator: D,
}

impl<W: io::Write, D: RecordDecorator> Serializer<W, D> {
    pub fn new(io: W, d: D) -> Self {
        Serializer {
            io: io,
            decorator: d,
        }
    }

    pub fn print_comma(&mut self) -> io::Result<()> {
        try!(self.decorator.fmt_separator(&mut self.io, &|io: &mut io::Write| write!(io, ", ")));
        Ok(())
    }

    pub fn finish(self) -> (W, D) {
        (self.io, self.decorator)
    }
}

macro_rules! s(
    ($s:expr, $k:expr, $v:expr) => {
        try!($s.decorator.fmt_key(&mut $s.io, &|io: &mut io::Write| write!(io, "{}", $k)));
        try!($s.decorator.fmt_separator(&mut $s.io, &|io: &mut io::Write| write!(io, ": ")));
        try!($s.decorator.fmt_value(&mut $s.io, &|io: &mut io::Write| write!(io, "{}", $v)));
    };
);

impl<W: io::Write, D: RecordDecorator> ser::Serializer for Serializer<W, D> {
    fn emit_none(&mut self, key: &str) -> ser::Result {
        s!(self, key, "None");
        Ok(())
    }

    fn emit_unit(&mut self, key: &str) -> ser::Result {
        s!(self, key, "()");
        Ok(())
    }

    fn emit_bool(&mut self, key: &str, val: bool) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_char(&mut self, key: &str, val: char) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_usize(&mut self, key: &str, val: usize) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_isize(&mut self, key: &str, val: isize) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_u8(&mut self, key: &str, val: u8) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_i8(&mut self, key: &str, val: i8) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_u16(&mut self, key: &str, val: u16) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_i16(&mut self, key: &str, val: i16) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_u32(&mut self, key: &str, val: u32) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_i32(&mut self, key: &str, val: i32) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_f32(&mut self, key: &str, val: f32) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_u64(&mut self, key: &str, val: u64) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_i64(&mut self, key: &str, val: i64) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_f64(&mut self, key: &str, val: f64) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_str(&mut self, key: &str, val: &str) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_arguments(&mut self, key: &str, val: &fmt::Arguments) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
}
