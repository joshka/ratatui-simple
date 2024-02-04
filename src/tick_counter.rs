use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use strum::{Display, EnumIs};

/// A simple example to show how how to break up the UI into smaller widgets.
///
/// This is a simple counter that increments at a regular interval. It can be paused and reset.
#[derive(Debug, Default)]
pub struct TickCounter {
    mode: Mode,
    tick_count: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Display, EnumIs)]
enum Mode {
    #[default]
    Running,
    Paused,
}

impl TickCounter {
    /// Handls a key event. Returns `Some(event)` if the event is not handled.
    pub fn handle_key_event(&mut self, event: KeyEvent) -> Option<KeyEvent> {
        match event.code {
            KeyCode::Char('r') => self.reset(),
            KeyCode::Char(' ') => self.toggle_pause(),
            _ => return Some(event),
        }
        None
    }

    pub fn toggle_pause(&mut self) {
        self.mode = match self.mode {
            Mode::Running => Mode::Paused,
            Mode::Paused => Mode::Running,
        };
    }

    pub fn tick(&mut self) {
        if self.mode.is_running() {
            self.tick_count += 1;
        }
    }

    pub fn reset(&mut self) {
        self.tick_count = 0;
    }

    /// Returns the current mode as a string.
    ///
    /// Used by the root widget to display the current mode in the header
    pub fn mode(&self) -> String {
        self.mode.to_string()
    }
}

impl Widget for &TickCounter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::from(format!("Ticks: {}", self.tick_count)).render(area, buf);
    }
}
