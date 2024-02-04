use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};

use crate::{
    events::{Event, EventHandler},
    tui::TerminalGuard,
};

#[derive(Debug, Default)]
pub struct App {
    mode: Mode,
    tick_count: usize,
}

/// Application mode.
///
/// This is used to control the state of the application. Your app might use this to control what
/// screen is being displayed, or to control the flow of the application.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// The default mode.
    #[default]
    Default,

    /// The application is paused.
    ///
    /// This is an example of how you might use the mode to control the flow of the application.
    Pause,

    /// The application is in the process of quitting.
    Quit,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Action {
    #[default]
    None,
    ResetTicks,
    TogglePause,
    Exit,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    /// Runs the main loop.
    pub fn run(&mut self, terminal: &mut TerminalGuard, events: &mut EventHandler) -> Result<()> {
        while self.is_running() {
            self.draw(terminal)?;
            self.handle_events(events)?;
        }
        Ok(())
    }

    /// Returns true if the application is running.
    pub fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    /// Draw the user interface.
    pub fn draw(&self, terminal: &mut TerminalGuard) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;
        Ok(())
    }

    /// Handle events.
    pub fn handle_events(&mut self, events: &mut EventHandler) -> Result<()> {
        match events.next()? {
            Event::Tick => self.tick(),
            Event::KeyPress(event) => match event.into() {
                Action::Exit => self.quit(),
                Action::ResetTicks => self.reset_ticks(),
                Action::TogglePause => self.toggle_pause(),
                Action::None => {}
            },
            Event::Mouse(_event) => {}
            Event::Resize(_size) => {}
            Event::Ignored => unreachable!(),
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// This is a good place to handle animations and other time-based events.
    pub fn tick(&mut self) {
        if self.mode == Mode::Pause {
            return;
        }
        self.tick_count += 1;
    }

    /// Reset the tick count.
    pub fn reset_ticks(&mut self) {
        self.tick_count = 0;
    }

    /// Toggle pause.
    pub fn toggle_pause(&mut self) {
        self.mode = match self.mode {
            Mode::Default => Mode::Pause,
            Mode::Pause => Mode::Default,
            Mode::Quit => Mode::Quit,
        };
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.mode = Mode::Quit;
    }
}

impl From<KeyEvent> for Action {
    fn from(key_event: KeyEvent) -> Self {
        use KeyCode::*;
        match key_event.code {
            Char('q') | Esc => Self::Exit,
            Char('c') if key_event.modifiers == KeyModifiers::CONTROL => Self::Exit,
            Char('r') => Self::ResetTicks,
            Char(' ') => Self::TogglePause,
            // You can add more key handlers here.
            _ => Self::None,
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // This is where you add new widgets. See the following resources:
        // - https://ratatui.rs/how-to/widgets/ TODO: (make concepts/widgets/ a good jumping off point)
        // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
        // - https://github.com/ratatui-org/ratatui/tree/master/examples

        use Constraint::*;
        let [header, body, footer] = Layout::vertical([Length(1), Fill(1), Length(1)]).areas(area);

        // header
        let header_fg = tailwind::SLATE.c300;
        let header_bg = tailwind::AMBER.c900;
        Line::from("Ratatui Template")
            .style((header_fg, header_bg, Modifier::BOLD))
            .render(header, buf);

        Body::new("Hello, Ratatui!", self.tick_count).render(body, buf);

        // footer
        Line::from("Quit: q, Reset: r, Pause: Space")
            .style((header_fg, header_bg, Modifier::BOLD))
            .render(footer, buf);
    }
}

/// A simple example to show how how to break up the UI into smaller widgets.
struct Body {
    text: &'static str,
    tick_count: usize,
}

impl Body {
    fn new(text: &'static str, tick_count: usize) -> Self {
        Self { text, tick_count }
    }
}

impl Widget for &Body {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let line_count = self.text.lines().count() as u16;
        let [text_area, tick_count_area] =
            Layout::vertical([Constraint::Length(line_count), Constraint::Length(1)]).areas(area);
        Paragraph::new(self.text).render(text_area, buf);
        Line::from(format!("Ticks: {}", self.tick_count)).render(tick_count_area, buf);
    }
}
