//! Monitor profile management.

use crate::hyprland::{Monitor, Position, Resolution, Transform};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Monitor config for profile (serializable).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonitorConfig {
    pub name: String,
    #[serde(default)]
    pub resolution: Option<String>,
    #[serde(default)]
    pub refresh_rate: Option<f32>,
    pub position: [i32; 2],
    #[serde(default = "one_f32")]
    pub scale: f32,
    #[serde(default)]
    pub transform: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub primary: bool,
}

fn one_f32() -> f32 {
    1.0
}
fn default_true() -> bool {
    true
}

/// Monitor configuration profile.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub description: Option<String>,
    pub monitors: Vec<MonitorConfig>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub modified_at: Option<String>,
}

impl Profile {
    /// Create from current monitors.
    pub fn from_monitors(name: String, description: Option<String>, monitors: &[Monitor]) -> Self {
        let now = Utc::now().to_rfc3339();
        let monitors = monitors
            .iter()
            .map(|m| MonitorConfig {
                name: m.name.clone(),
                resolution: Some(m.resolution.to_string()),
                refresh_rate: Some(m.refresh_rate),
                position: [m.position.x, m.position.y],
                scale: m.scale,
                transform: m.transform.to_hyprland_str().to_string(),
                enabled: m.enabled,
                primary: m.primary,
            })
            .collect();
        Profile {
            name,
            description,
            monitors,
            created_at: Some(now.clone()),
            modified_at: Some(now),
        }
    }

    /// Convert to Monitor list (requires merging with current monitor list for resolutions).
    pub fn to_monitors(&self, current: &[Monitor]) -> Vec<Monitor> {
        self.monitors
            .iter()
            .filter_map(|mc| {
                let cur = current.iter().find(|m| m.name == mc.name)?;
                let (res, refresh): (Resolution, f32) = mc
                    .resolution
                    .as_ref()
                    .and_then(|r| parse_res(r))
                    .map(|(res, hz)| (res, mc.refresh_rate.unwrap_or(hz)))
                    .unwrap_or((cur.resolution.clone(), mc.refresh_rate.unwrap_or(cur.refresh_rate)));
                Some(Monitor {
                    name: mc.name.clone(),
                    description: cur.description.clone(),
                    position: Position {
                        x: mc.position[0],
                        y: mc.position[1],
                    },
                    resolution: res,
                    available_resolutions: cur.available_resolutions.clone(),
                    refresh_rate: refresh,
                    available_refresh_rates: cur.available_refresh_rates.clone(),
                    scale: mc.scale,
                    transform: Transform::from_hyprland_str(&mc.transform),
                    enabled: mc.enabled,
                    primary: mc.primary,
                })
            })
            .collect()
    }
}

fn parse_res(s: &str) -> Option<(Resolution, f32)> {
    let (res, hz) = s.split_once('@').map(|(a, b)| (a, b.parse().ok()))?;
    let (w, h) = res.split_once('x')?;
    Some((
        Resolution {
            width: w.parse().ok()?,
            height: h.parse().ok()?,
        },
        hz.unwrap_or(60.0),
    ))
}

/// Load profile from path.
pub fn load_profile(path: &Path) -> anyhow::Result<Profile> {
    let s = std::fs::read_to_string(path)?;
    let p: Profile = toml::from_str(&s)?;
    Ok(p)
}

/// Save profile to path.
pub fn save_profile(path: &Path, profile: &Profile) -> anyhow::Result<()> {
    let s = toml::to_string_pretty(profile)?;
    std::fs::write(path, s)?;
    Ok(())
}

/// List profile names in directory.
pub fn list_profiles(dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return vec![];
    };
    entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|x| x == "toml"))
        .filter_map(|e| e.path().file_stem().and_then(|s| s.to_str().map(String::from)))
        .collect()
}
