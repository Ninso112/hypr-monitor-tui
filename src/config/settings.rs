//! Application settings (themes, defaults).

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

/// Application color theme (Omarchy-compatible).
#[derive(Clone, Debug)]
pub struct Theme {
    /// Background color
    pub bg: Color,
    /// Foreground/text color
    pub fg: Color,
    /// Dimmed text color
    pub fg_dim: Color,
    /// Primary accent color
    pub accent: Color,
    /// Secondary accent color
    pub accent_secondary: Color,
    /// Success/positive color
    pub success: Color,
    /// Warning color
    pub warning: Color,
    /// Error/danger color
    pub error: Color,
    /// Border color
    pub border: Color,
    /// Active border color
    pub border_active: Color,
    /// Selection background
    pub selection: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: Color::Rgb(26, 27, 38),
            fg: Color::Rgb(192, 202, 245),
            fg_dim: Color::Rgb(86, 95, 137),
            accent: Color::Rgb(122, 162, 247),
            accent_secondary: Color::Rgb(187, 154, 247),
            success: Color::Rgb(158, 206, 106),
            warning: Color::Rgb(224, 175, 104),
            error: Color::Rgb(247, 118, 142),
            border: Color::Rgb(65, 72, 104),
            border_active: Color::Rgb(122, 162, 247),
            selection: Color::Rgb(40, 52, 87),
        }
    }
}

/// Theme config (hex strings for serialization).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub accent: Option<String>,
    pub accent_secondary: Option<String>,
    pub success: Option<String>,
    pub warning: Option<String>,
    pub error: Option<String>,
}

impl ThemeConfig {
    /// Parse hex color "#rrggbb" to Color.
    pub fn parse_hex(s: &str) -> Option<Color> {
        let s = s.trim().trim_start_matches('#');
        if s.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some(Color::Rgb(r, g, b))
    }

    /// Apply config to theme.
    pub fn apply_to(&self, theme: &mut Theme) {
        if let Some(ref s) = self.accent {
            if let Some(c) = Self::parse_hex(s) {
                theme.accent = c;
                theme.border_active = c;
            }
        }
        if let Some(ref s) = self.accent_secondary {
            if let Some(c) = Self::parse_hex(s) {
                theme.accent_secondary = c;
            }
        }
        if let Some(ref s) = self.success {
            if let Some(c) = Self::parse_hex(s) {
                theme.success = c;
            }
        }
        if let Some(ref s) = self.warning {
            if let Some(c) = Self::parse_hex(s) {
                theme.warning = c;
            }
        }
        if let Some(ref s) = self.error {
            if let Some(c) = Self::parse_hex(s) {
                theme.error = c;
            }
        }
    }
}

/// General app settings.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GeneralSettings {
    #[serde(default)]
    pub auto_apply: bool,
    #[serde(default = "default_preview_timeout")]
    pub preview_timeout: u64,
    #[serde(default = "default_scale_step")]
    pub scale_step: f32,
}

fn default_preview_timeout() -> u64 {
    10
}
fn default_scale_step() -> f32 {
    0.25
}

/// Keybindings (optional overrides).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct KeybindingsConfig {
    pub quit: Option<String>,
    pub apply: Option<String>,
    pub save: Option<String>,
    pub help: Option<String>,
}

/// Root config file structure.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub general: GeneralSettings,
    #[serde(default)]
    pub theme: ThemeConfig,
    #[serde(default)]
    pub keybindings: KeybindingsConfig,
}
