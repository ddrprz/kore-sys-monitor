use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    CyberCyan,
    CatppuccinMocha,
    Dracula,
    MonochromeMatrix,
}

impl ThemeVariant {
    pub fn next(&self) -> Self {
        match self {
            ThemeVariant::CyberCyan => ThemeVariant::CatppuccinMocha,
            ThemeVariant::CatppuccinMocha => ThemeVariant::Dracula,
            ThemeVariant::Dracula => ThemeVariant::MonochromeMatrix,
            ThemeVariant::MonochromeMatrix => ThemeVariant::CyberCyan,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ThemeVariant::CyberCyan => "Cyber Cyan",
            ThemeVariant::CatppuccinMocha => "Catppuccin Mocha",
            ThemeVariant::Dracula => "Dracula",
            ThemeVariant::MonochromeMatrix => "Monochrome Matrix",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub variant: ThemeVariant,
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub critical: Color,
    pub border_active: Color,
    pub border_inactive: Color,
    pub text_muted: Color,
}

impl Theme {
    pub fn from_variant(variant: ThemeVariant) -> Self {
        match variant {
            ThemeVariant::CyberCyan => Self {
                variant,
                primary: Color::Cyan,
                secondary: Color::Magenta,
                success: Color::Green,
                warning: Color::Yellow,
                critical: Color::Red,
                border_active: Color::Cyan,
                border_inactive: Color::DarkGray,
                text_muted: Color::DarkGray,
            },
            ThemeVariant::CatppuccinMocha => Self {
                variant,
                primary: Color::Rgb(180, 190, 254),    // Lavender
                secondary: Color::Rgb(203, 166, 247),  // Mauve
                success: Color::Rgb(166, 227, 161),    // Green
                warning: Color::Rgb(249, 226, 175),    // Yellow/Peach
                critical: Color::Rgb(243, 139, 168),   // Red
                border_active: Color::Rgb(180, 190, 254),
                border_inactive: Color::Rgb(88, 91, 112),
                text_muted: Color::Rgb(147, 153, 178),
            },
            ThemeVariant::Dracula => Self {
                variant,
                primary: Color::Rgb(189, 147, 249),    // Purple
                secondary: Color::Rgb(255, 121, 198),  // Pink
                success: Color::Rgb(80, 250, 123),     // Green
                warning: Color::Rgb(241, 250, 140),    // Yellow
                critical: Color::Rgb(255, 85, 85),      // Red
                border_active: Color::Rgb(189, 147, 249),
                border_inactive: Color::Rgb(98, 114, 164), // Comment
                text_muted: Color::Rgb(98, 114, 164),
            },
            ThemeVariant::MonochromeMatrix => Self {
                variant,
                primary: Color::Green,
                secondary: Color::LightGreen,
                success: Color::Green,
                warning: Color::Yellow,
                critical: Color::Red,
                border_active: Color::Green,
                border_inactive: Color::DarkGray,
                text_muted: Color::DarkGray,
            },
        }
    }
}
