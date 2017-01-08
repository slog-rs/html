#[derive(Clone, Copy)]
/// Formatting style
pub struct Style {
    /// Optionally use custom color (hexadecimal color code)
    pub color: Option<&'static str>,
    /// Use bold font
    pub bold: bool,
    /// Use italic font
    pub italic: bool,
    /// Use custom CSS style
    ///
    /// Example: Some("background-color: #ffff7f;")
    pub custom: Option<&'static str>,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            color: None,
            bold: false,
            italic: false,
            custom: None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct StyleTable {
    pub level: Style,
    pub timestamp: Style,
    pub message: Style,
    pub key: Style,
    pub value: Style,
    pub separator: Style,
}

impl Default for StyleTable {
    fn default() -> Self {
        StyleTable {
            level: Style {
                color: None,
                bold: false,
                italic: false,
                custom: None,
            },
            timestamp: Style {
                color: None,
                bold: false,
                italic: false,
                custom: None,
            },
            message: Style {
                color: None,
                bold: true,
                italic: false,
                custom: None,
            },
            key: Style {
                color: Some("55557f"),
                bold: true,
                italic: false,
                custom: None,
            },
            value: Style {
                color: None,
                bold: false,
                italic: false,
                custom: None,
            },
            separator: Style {
                color: None,
                bold: false,
                italic: false,
                custom: None,
            },
        }
    }
}
