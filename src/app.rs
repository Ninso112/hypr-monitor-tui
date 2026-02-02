//! Main application logic and state management.

use crate::config::Theme;
use crate::hyprland::Monitor;
use std::path::PathBuf;

/// Confirmation action type.
#[derive(Clone, Debug, PartialEq)]
pub enum ConfirmAction {
    Quit,
    Apply,
    Save,
    Reset,
    DeleteProfile(String),
}

/// Fields that can be edited.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditField {
    Resolution,
    RefreshRate,
    Scale,
    Transform,
    Primary,
    Enabled,
}

impl EditField {
    pub fn next(self) -> Self {
        match self {
            EditField::Resolution => EditField::RefreshRate,
            EditField::RefreshRate => EditField::Scale,
            EditField::Scale => EditField::Transform,
            EditField::Transform => EditField::Primary,
            EditField::Primary => EditField::Enabled,
            EditField::Enabled => EditField::Resolution,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            EditField::Resolution => EditField::Enabled,
            EditField::RefreshRate => EditField::Resolution,
            EditField::Scale => EditField::RefreshRate,
            EditField::Transform => EditField::Scale,
            EditField::Primary => EditField::Transform,
            EditField::Enabled => EditField::Primary,
        }
    }
}

/// Application operating mode.
#[derive(Clone, Debug, PartialEq)]
pub enum AppMode {
    /// Normal navigation mode
    Normal,
    /// Moving a monitor in the grid
    Moving,
    /// Editing monitor settings
    Editing { field: EditField },
    /// Showing help overlay
    Help,
    /// Showing confirmation dialog
    Confirm {
        action: ConfirmAction,
        message: String,
    },
    /// Profile selection
    ProfileSelect,
}

/// Snapshot of app state for undo.
#[derive(Clone)]
pub struct AppStateSnapshot {
    pub monitors: Vec<Monitor>,
    pub selected_monitor: usize,
}

/// Main application state.
pub struct App {
    /// All detected monitors
    pub monitors: Vec<Monitor>,
    /// Currently selected monitor index
    pub selected_monitor: usize,
    /// Current application mode
    pub mode: AppMode,
    /// Focus: grid (false) or settings panel (true)
    pub focus_settings: bool,
    /// Flag for unsaved changes
    pub unsaved_changes: bool,
    /// Configuration file path
    pub config_path: PathBuf,
    /// Change history for undo
    pub history: Vec<AppStateSnapshot>,
    /// Available profile names
    pub profiles: Vec<String>,
    /// Theme
    pub theme: Theme,
    /// Status message
    pub status_message: Option<String>,
    /// Error message
    pub error_message: Option<String>,
}

impl App {
    pub fn new(monitors: Vec<Monitor>, config_path: PathBuf, theme: Theme) -> Self {
        let profiles = crate::config::list_profiles(
            config_path
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .join("profiles")
                .as_path(),
        );
        Self {
            monitors,
            selected_monitor: 0,
            mode: AppMode::Normal,
            focus_settings: false,
            unsaved_changes: false,
            config_path,
            history: Vec::new(),
            profiles,
            theme,
            status_message: None,
            error_message: None,
        }
    }

    pub fn selected(&self) -> Option<&Monitor> {
        self.monitors.get(self.selected_monitor)
    }

    pub fn selected_mut(&mut self) -> Option<&mut Monitor> {
        self.monitors.get_mut(self.selected_monitor)
    }

    pub fn push_history(&mut self) {
        self.history.push(AppStateSnapshot {
            monitors: self.monitors.clone(),
            selected_monitor: self.selected_monitor,
        });
        if self.history.len() > 50 {
            self.history.remove(0);
        }
    }

    pub fn undo(&mut self) -> bool {
        if let Some(snap) = self.history.pop() {
            self.monitors = snap.monitors;
            self.selected_monitor = snap.selected_monitor.min(self.monitors.len().saturating_sub(1));
            self.unsaved_changes = true;
            true
        } else {
            false
        }
    }

    pub fn set_status(&mut self, msg: String) {
        self.status_message = Some(msg);
    }

    pub fn set_error(&mut self, msg: String) {
        self.error_message = Some(msg);
    }

    pub fn clear_messages(&mut self) {
        self.status_message = None;
        self.error_message = None;
    }
}
