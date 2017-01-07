use slog::Level;

/// Hexadecimal color codes
pub struct ColorPalette {
    /// Color for critical messages
    pub critical: &'static str,

    /// Color for error messages
    pub error: &'static str,

    /// Color for warning messages
    pub warning: &'static str,

    /// Color for info messages
    pub info: &'static str,

    /// Color for debug messages
    pub debug: &'static str,

    /// Color for trace messages
    pub trace: &'static str,
}

impl ColorPalette {
    /// Returns the corresponding color for an slog level
    pub fn level_to_color(&self, level: Level) -> &'static str {
        use slog::Level::*;
        match level {
            Critical => self.critical,
            Error => self.error,
            Warning => self.warning,
            Info => self.info,
            Debug => self.debug,
            Trace => self.trace,
        }
    }
}

impl Default for ColorPalette {
    /// ```
    /// critical: "ff0000"
    /// error: "ff5500"
    /// warning: "ffaa00"
    /// info: "55aa00"
    /// debug: "aaaa7f"
    /// trace: "55557f"
    /// ```
    fn default() -> Self {
        ColorPalette {
            critical: "ff0000",
            error: "ff5500",
            warning: "ffaa00",
            info: "55aa00",
            debug: "aaaa7f",
            trace: "55557f",
        }
    }
}
