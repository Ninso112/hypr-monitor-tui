//! Event handling (keyboard, mouse, resize).

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::sync::mpsc;
use std::time::Duration;

/// Application events.
#[derive(Clone, Debug)]
pub enum AppEvent {
    /// Keyboard input
    Key(KeyEvent),
    /// Mouse input
    Mouse(MouseEvent),
    /// Terminal resize
    Resize(u16, u16),
    /// Tick for animations/updates
    Tick,
    /// Hyprland monitor change notification
    MonitorChange,
}

/// Event handler.
pub struct EventHandler {
    receiver: mpsc::Receiver<AppEvent>,
}

impl EventHandler {
    /// Create new event handler; spawns a thread that polls crossterm and sends Tick periodically.
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, receiver) = mpsc::channel();
        std::thread::spawn(move || loop {
            if event::poll(tick_rate).unwrap_or(false) {
                if let Ok(ev) = event::read() {
                    match ev {
                        CrosstermEvent::Key(key) => {
                            let _ = tx.send(AppEvent::Key(key));
                        }
                        CrosstermEvent::Mouse(m) => {
                            let _ = tx.send(AppEvent::Mouse(m));
                        }
                        CrosstermEvent::Resize(w, h) => {
                            let _ = tx.send(AppEvent::Resize(w, h));
                        }
                        _ => {}
                    }
                }
            }
            let _ = tx.send(AppEvent::Tick);
        });
        Self { receiver }
    }

    /// Receive next event (blocking).
    pub fn recv_event(&mut self) -> Result<AppEvent, mpsc::RecvError> {
        self.receiver.recv()
    }
}
