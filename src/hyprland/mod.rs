//! Hyprland integration (IPC, monitor data, config generation).

mod config;
mod ipc;
mod monitor;

pub use config::{generate_config, generate_config_with_workspaces};
pub use ipc::HyprlandClient;
pub use monitor::{Mode, Monitor, Position, Resolution, Transform};
