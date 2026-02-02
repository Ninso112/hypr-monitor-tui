//! Monitor data structures for Hyprland.

use serde::{Deserialize, Serialize};

/// Represents a physical monitor.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Monitor {
    /// Monitor name (e.g., "DP-1", "HDMI-A-1")
    pub name: String,
    /// Human-readable description (manufacturer + model)
    pub description: String,
    /// Position in the virtual screen space
    pub position: Position,
    /// Current resolution
    pub resolution: Resolution,
    /// Available resolutions
    pub available_resolutions: Vec<Resolution>,
    /// Current refresh rate in Hz
    pub refresh_rate: f32,
    /// Available refresh rates
    pub available_refresh_rates: Vec<f32>,
    /// Display scale factor
    pub scale: f32,
    /// Display transform (rotation/flip)
    pub transform: Transform,
    /// Whether the monitor is enabled
    pub enabled: bool,
    /// Whether this is the primary monitor
    pub primary: bool,
}

/// Screen position.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Screen resolution.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

/// Display transform options.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum Transform {
    #[default]
    Normal,
    Rotate90,
    Rotate180,
    Rotate270,
    Flipped,
    Flipped90,
    Flipped180,
    Flipped270,
}

impl Transform {
    /// Hyprland transform string.
    pub fn to_hyprland_str(&self) -> &'static str {
        match self {
            Transform::Normal => "0",
            Transform::Rotate90 => "1",
            Transform::Rotate180 => "2",
            Transform::Rotate270 => "3",
            Transform::Flipped => "4",
            Transform::Flipped90 => "5",
            Transform::Flipped180 => "6",
            Transform::Flipped270 => "7",
        }
    }

    /// Parse from Hyprland string.
    pub fn from_hyprland_str(s: &str) -> Self {
        match s {
            "1" => Transform::Rotate90,
            "2" => Transform::Rotate180,
            "3" => Transform::Rotate270,
            "4" => Transform::Flipped,
            "5" => Transform::Flipped90,
            "6" => Transform::Flipped180,
            "7" => Transform::Flipped270,
            _ => Transform::Normal,
        }
    }

    /// Cycle to next transform.
    pub fn next(&self) -> Self {
        match self {
            Transform::Normal => Transform::Rotate90,
            Transform::Rotate90 => Transform::Rotate180,
            Transform::Rotate180 => Transform::Rotate270,
            Transform::Rotate270 => Transform::Flipped,
            Transform::Flipped => Transform::Flipped90,
            Transform::Flipped90 => Transform::Flipped180,
            Transform::Flipped180 => Transform::Flipped270,
            Transform::Flipped270 => Transform::Normal,
        }
    }
}

impl std::fmt::Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Transform::Normal => "Normal",
            Transform::Rotate90 => "90°",
            Transform::Rotate180 => "180°",
            Transform::Rotate270 => "270°",
            Transform::Flipped => "Flipped",
            Transform::Flipped90 => "Flipped 90°",
            Transform::Flipped180 => "Flipped 180°",
            Transform::Flipped270 => "Flipped 270°",
        };
        write!(f, "{}", s)
    }
}

/// Mode from Hyprland (resolution + refresh rate).
#[derive(Clone, Debug)]
pub struct Mode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: f32,
}
