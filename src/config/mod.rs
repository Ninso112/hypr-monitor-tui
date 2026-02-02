//! Configuration management.

mod profiles;
mod settings;

pub use profiles::{load_profile, list_profiles, save_profile, MonitorConfig, Profile};
pub use settings::{AppConfig, GeneralSettings, KeybindingsConfig, Theme, ThemeConfig};
