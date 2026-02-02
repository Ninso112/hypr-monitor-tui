//! IPC communication with Hyprland.

use crate::hyprland::monitor::{Mode, Monitor, Position, Resolution, Transform};
use anyhow::{Context, Result};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::str::FromStr;
use tracing::debug;

/// Resolve Hyprland command socket path (matches Hyprland: XDG_RUNTIME_DIR first, then /tmp).
fn command_socket_path() -> Option<std::path::PathBuf> {
    let sig = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").ok()?;
    let sock = std::path::Path::new("hypr").join(&sig).join(".socket.sock");
    if let Ok(runtime) = std::env::var("XDG_RUNTIME_DIR") {
        let p = std::path::Path::new(&runtime).join(&sock);
        if p.exists() {
            return Some(p);
        }
    }
    let p = std::path::Path::new("/tmp").join(&sock);
    if p.exists() {
        Some(p)
    } else {
        None
    }
}

/// Hyprland transform to our Transform.
fn hypr_transform_to_ours(t: hyprland::data::Transforms) -> Transform {
    use hyprland::data::Transforms;
    match t {
        Transforms::Normal => Transform::Normal,
        Transforms::Normal90 => Transform::Rotate90,
        Transforms::Normal180 => Transform::Rotate180,
        Transforms::Normal270 => Transform::Rotate270,
        Transforms::Flipped => Transform::Flipped,
        Transforms::Flipped90 => Transform::Flipped90,
        Transforms::Flipped180 => Transform::Flipped180,
        Transforms::Flipped270 => Transform::Flipped270,
    }
}

/// Hyprland IPC client wrapper.
pub struct HyprlandClient;

impl Default for HyprlandClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HyprlandClient {
    /// Create new client. Does not require async connection; hyprland-rs uses sync socket.
    pub fn new() -> Self {
        Self
    }

    /// Get all monitors with their current configuration.
    /// Uses our own socket path (XDG_RUNTIME_DIR first) so it works when Hyprland
    /// uses XDG_RUNTIME_DIR and hyprland-rs would look only in /tmp.
    pub fn get_monitors(&self) -> Result<Vec<Monitor>> {
        let path = command_socket_path()
            .context("Hyprland socket not found (is Hyprland running?)")?;
        let response = raw_socket_request(path, "j/monitors")?;
        let hypr_monitors: Vec<hyprland::data::Monitor> =
            serde_json::from_str(&response).context("Failed to parse monitors JSON")?;
        let mut monitors = Vec::new();
        for m in &hypr_monitors {
            let resolution = Resolution {
                width: u32::from(m.width),
                height: u32::from(m.height),
            };
            let position = Position { x: m.x, y: m.y };
            let transform = hypr_transform_to_ours(m.transform.clone());
            let res_list = vec![resolution.clone()];
            let hz_list = vec![m.refresh_rate];

            monitors.push(Monitor {
                name: m.name.clone(),
                description: m.description.clone(),
                position,
                resolution: resolution.clone(),
                available_resolutions: res_list,
                refresh_rate: m.refresh_rate,
                available_refresh_rates: hz_list,
                scale: m.scale,
                transform,
                enabled: m.dpms_status,
                primary: m.focused,
            });
        }
        if !monitors.is_empty() && !monitors.iter().any(|m| m.primary) {
            monitors[0].primary = true;
        }
        Ok(monitors)
    }

    /// Get available modes for a specific monitor (async placeholder for future API).
    pub async fn get_available_modes(&self, monitor_name: &str) -> Result<Vec<Mode>> {
        let _ = monitor_name;
        Ok(Vec::new())
    }

    /// Apply monitor configuration via keyword (uses our socket path).
    pub fn apply_monitor_config(&self, monitor: &Monitor) -> Result<()> {
        if !monitor.enabled {
            return Ok(());
        }
        let path = command_socket_path()
            .context("Hyprland socket not found")?;
        let spec = format!(
            "{},{}@{:.2},{}x{},{}",
            monitor.name,
            monitor.resolution,
            monitor.refresh_rate,
            monitor.position.x,
            monitor.position.y,
            monitor.scale
        );
        debug!("Setting monitor keyword: {}", spec);
        let _ = raw_socket_request(path, &format!("/keyword monitor {}", spec))
            .context("Failed to apply monitor configuration")?;
        Ok(())
    }

    /// Apply all monitor configurations (set keyword for each; last may win in some setups).
    pub fn apply_all(&self, monitors: &[Monitor]) -> Result<()> {
        let enabled: Vec<_> = monitors.iter().filter(|m| m.enabled).collect();
        for m in &enabled {
            self.apply_monitor_config(m)?;
        }
        Ok(())
    }

    /// Check if Hyprland is likely running (env set and socket exists).
    /// Checks both XDG_RUNTIME_DIR and /tmp to match Hyprland’s socket location.
    pub fn is_available() -> bool {
        command_socket_path().is_some()
    }
}

/// Send a raw command to the Hyprland socket and return the response.
fn raw_socket_request(path: std::path::PathBuf, command: &str) -> Result<String> {
    let mut stream = UnixStream::connect(&path).context("Failed to connect to Hyprland socket")?;
    stream
        .write_all(command.as_bytes())
        .context("Failed to write to socket")?;
    let mut response = Vec::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = stream.read(&mut buf).context("Failed to read from socket")?;
        if n == 0 {
            break;
        }
        response.extend_from_slice(&buf[..n]);
        if n < buf.len() {
            break;
        }
    }
    String::from_utf8(response).context("Invalid UTF-8 from socket")
}

/// Parse resolution@refresh string (for tests / fallback).
#[allow(dead_code)]
fn parse_resolution_refresh(s: &str) -> Result<(Resolution, f32)> {
    let s = s.trim();
    let (res_part, hz_part) = s
        .split_once('@')
        .map(|(a, b)| (a, Some(b)))
        .unwrap_or((s, None));
    let (w, h) = res_part
        .split_once('x')
        .context("Invalid resolution format")?;
    let width = u32::from_str(w.trim()).context("Invalid width")?;
    let height = u32::from_str(h.trim()).context("Invalid height")?;
    let refresh_rate = hz_part
        .and_then(|hz| f32::from_str(hz.trim()).ok())
        .unwrap_or(60.0);
    Ok((
        Resolution { width, height },
        refresh_rate,
    ))
}

/// Parse position string (for tests).
#[allow(dead_code)]
fn parse_position(s: &str) -> Result<(i32, i32)> {
    let s = s.trim();
    let (x, y) = s
        .split_once(',')
        .or_else(|| s.split_once('_').or_else(|| s.split_once('x')))
        .context("Invalid position format")?;
    let x = i32::from_str(x.trim()).context("Invalid x")?;
    let y = i32::from_str(y.trim()).context("Invalid y")?;
    Ok((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_resolution_refresh() {
        let (res, hz) = parse_resolution_refresh("2560x1440@144").unwrap();
        assert_eq!(res.width, 2560);
        assert_eq!(res.height, 1440);
        assert!((hz - 144.0).abs() < 0.01);
    }

    #[test]
    fn test_parse_position() {
        let (x, y) = parse_position("2560_0").unwrap();
        assert_eq!(x, 2560);
        assert_eq!(y, 0);
    }
}
