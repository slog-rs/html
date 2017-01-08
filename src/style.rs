#[derive(Clone, Copy)]
pub struct Style {
    pub color: Option<&'static str>,
    pub bold: bool,
}

#[derive(Clone, Copy)]
pub struct StyleTable {
    pub timestamp: Style,
    pub message: Style,
    pub key: Style,
    pub value: Style,
    pub separator: Style,
}

impl Default for StyleTable {
    fn default() -> Self {
        StyleTable {
            timestamp: Style {
                color: None,
                bold: false,
            },
            message: Style {
                color: None,
                bold: true,
            },
            key: Style {
                color: Some("55557f"),
                bold: true,
            },
            value: Style {
                color: None,
                bold: false,
            },
            separator: Style {
                color: None,
                bold: false,
            },
        }
    }
}
