use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    events::{Event, EventHandler},
    root::Root,
    tui::TerminalGuard,
};

#[derive(Debug, Default)]
pub struct App {
    mode: Mode,
    root: Root,
}

/// Application mode is used to control the state of the application.
///
/// Your app might use this to control which screen to display, or to control the flow of the app.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// The app is running normally.
    #[default]
    Running,
    /// The application is in the process of quitting.
    Quit,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, terminal: &mut TerminalGuard, events: &mut EventHandler) -> Result<()> {
        while self.is_running() {
            self.draw(terminal)?;
            self.handle_events(events)?;
        }
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.mode == Mode::Running
    }

    pub fn draw(&self, terminal: &mut TerminalGuard) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(&self.root, frame.size()))?;
        Ok(())
    }

    pub fn handle_events(&mut self, events: &mut EventHandler) -> Result<()> {
        match events.next()? {
            Event::Tick => self.root.tick(),
            Event::KeyPress(event) => self.handle_key_event(event),
            Event::Mouse(_event) => {}
            Event::Resize(_size) => {}
            Event::Ignored => unreachable!(),
        }
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        use KeyCode::*;
        // handle global key events
        if event.code == Char('c') && event.modifiers == KeyModifiers::CONTROL {
            // Ctrl+C doesn't trigger a SIGINT signal in raw mode, so we handle it manually.
            self.quit();
            return;
        }

        let unhandled_event = self.root.handle_key_event(event);
        if let Some(event) = unhandled_event {
            match event.code {
                Char('q') | Esc => self.quit(),
                _ => {}
            }
        }
    }

    pub fn quit(&mut self) {
        self.mode = Mode::Quit;
    }
}
