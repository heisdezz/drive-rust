use gpui_kit::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ThemeMode {
    #[default]
    ZedDark,
    CatppuccinMocha,
    TokyoNight,
    Nord,
    ZedLight,
}

impl ThemeMode {
    pub fn name(&self) -> &'static str {
        match self {
            ThemeMode::ZedDark => "Zed Dark",
            ThemeMode::CatppuccinMocha => "Catppuccin Mocha",
            ThemeMode::TokyoNight => "Tokyo Night",
            ThemeMode::Nord => "Nord Arctic",
            ThemeMode::ZedLight => "Zed Light",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ThemeMode::ZedDark => "◑",
            ThemeMode::CatppuccinMocha => "◐",
            ThemeMode::TokyoNight => "◒",
            ThemeMode::Nord => "◓",
            ThemeMode::ZedLight => "○",
        }
    }

    pub fn all() -> &'static [ThemeMode] {
        &[
            ThemeMode::ZedDark,
            ThemeMode::CatppuccinMocha,
            ThemeMode::TokyoNight,
            ThemeMode::Nord,
            ThemeMode::ZedLight,
        ]
    }
}

/// Dynamic semantic color palette for Rust Drive.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AppTheme {
    pub mode: ThemeMode,
    pub is_dark: bool,

    // Backgrounds
    pub bg_canvas: Rgba,
    pub bg_sidebar: Rgba,
    pub bg_header: Rgba,
    pub bg_card: Rgba,
    pub bg_card_hover: Rgba,
    pub bg_input: Rgba,
    pub bg_pill: Rgba,
    pub bg_pill_active: Rgba,

    // Borders
    pub border_subtle: Rgba,
    pub border_card: Rgba,
    pub border_active: Rgba,

    // Typography
    pub text_primary: Rgba,
    pub text_secondary: Rgba,
    pub text_muted: Rgba,

    // Accents & Status
    pub accent: Rgba,
    pub accent_hover: Rgba,
    pub accent_success: Rgba,
    pub accent_warning: Rgba,
    pub accent_danger: Rgba,
}

impl AppTheme {
    pub fn name(&self) -> &'static str {
        self.mode.name()
    }

    pub fn from_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::ZedDark => Self {
                mode,
                is_dark: true,
                bg_canvas: rgb(0x15171c),
                bg_sidebar: rgb(0x111317),
                bg_header: rgb(0x131519),
                bg_card: rgb(0x181a20),
                bg_card_hover: rgb(0x1f232b),
                bg_input: rgb(0x1c1e24),
                bg_pill: rgb(0x181a20),
                bg_pill_active: rgb(0x22262e),
                border_subtle: rgb(0x1e2025),
                border_card: rgb(0x23262d),
                border_active: rgb(0x384152),
                text_primary: rgb(0xf4f4f5),
                text_secondary: rgb(0xa1a1aa),
                text_muted: rgb(0x71717a),
                accent: rgb(0x3b82f6),
                accent_hover: rgb(0x60a5fa),
                accent_success: rgb(0x10b981),
                accent_warning: rgb(0xf59e0b),
                accent_danger: rgb(0xf87171),
            },
            ThemeMode::CatppuccinMocha => Self {
                mode,
                is_dark: true,
                bg_canvas: rgb(0x1e1e2e),
                bg_sidebar: rgb(0x181825),
                bg_header: rgb(0x11111b),
                bg_card: rgb(0x313244),
                bg_card_hover: rgb(0x45475a),
                bg_input: rgb(0x1e1e2e),
                bg_pill: rgb(0x313244),
                bg_pill_active: rgb(0x585b70),
                border_subtle: rgb(0x313244),
                border_card: rgb(0x45475a),
                border_active: rgb(0xcba6f7),
                text_primary: rgb(0xcdd6f4),
                text_secondary: rgb(0xa6adc8),
                text_muted: rgb(0x6c7086),
                accent: rgb(0x89b4fa),
                accent_hover: rgb(0xb4befe),
                accent_success: rgb(0xa6e3a1),
                accent_warning: rgb(0xf9e2af),
                accent_danger: rgb(0xf38ba8),
            },
            ThemeMode::TokyoNight => Self {
                mode,
                is_dark: true,
                bg_canvas: rgb(0x1a1b26),
                bg_sidebar: rgb(0x16161e),
                bg_header: rgb(0x13131a),
                bg_card: rgb(0x202230),
                bg_card_hover: rgb(0x272a3c),
                bg_input: rgb(0x24283b),
                bg_pill: rgb(0x202231),
                bg_pill_active: rgb(0x2f354f),
                border_subtle: rgb(0x282c3f),
                border_card: rgb(0x2f344d),
                border_active: rgb(0x7aa2f7),
                text_primary: rgb(0xc0caf5),
                text_secondary: rgb(0x9aa5ce),
                text_muted: rgb(0x565f89),
                accent: rgb(0x7aa2f7),
                accent_hover: rgb(0xbb9af7),
                accent_success: rgb(0x9ece6a),
                accent_warning: rgb(0xe0af68),
                accent_danger: rgb(0xf7768e),
            },
            ThemeMode::Nord => Self {
                mode,
                is_dark: true,
                bg_canvas: rgb(0x2e3440),
                bg_sidebar: rgb(0x242933),
                bg_header: rgb(0x222630),
                bg_card: rgb(0x3b4252),
                bg_card_hover: rgb(0x434c5e),
                bg_input: rgb(0x3b4252),
                bg_pill: rgb(0x353b49),
                bg_pill_active: rgb(0x4c566a),
                border_subtle: rgb(0x3b4252),
                border_card: rgb(0x434c5e),
                border_active: rgb(0x88c0d0),
                text_primary: rgb(0xeceff4),
                text_secondary: rgb(0xd8dee9),
                text_muted: rgb(0x7c818c),
                accent: rgb(0x88c0d0),
                accent_hover: rgb(0x81a1c1),
                accent_success: rgb(0xa3be8c),
                accent_warning: rgb(0xebcb8b),
                accent_danger: rgb(0xbf616a),
            },
            ThemeMode::ZedLight => Self {
                mode,
                is_dark: false,
                bg_canvas: rgb(0xf8fafc),
                bg_sidebar: rgb(0xf1f5f9),
                bg_header: rgb(0xffffff),
                bg_card: rgb(0xffffff),
                bg_card_hover: rgb(0xf8fafc),
                bg_input: rgb(0xe2e8f0),
                bg_pill: rgb(0xe2e8f0),
                bg_pill_active: rgb(0xdbeafe),
                border_subtle: rgb(0xe2e8f0),
                border_card: rgb(0xcbd5e1),
                border_active: rgb(0x3b82f6),
                text_primary: rgb(0x0f172a),
                text_secondary: rgb(0x475569),
                text_muted: rgb(0x94a3b8),
                accent: rgb(0x2563eb),
                accent_hover: rgb(0x1d4ed8),
                accent_success: rgb(0x16a34a),
                accent_warning: rgb(0xd97706),
                accent_danger: rgb(0xdc2626),
            },
        }
    }
}

impl Default for AppTheme {
    fn default() -> Self {
        Self::from_mode(ThemeMode::default())
    }
}
