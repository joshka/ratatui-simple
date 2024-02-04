use crossterm::event::KeyEvent;
use ratatui::{prelude::*, style::palette::tailwind};

use crate::tick_counter::TickCounter;

/// The root widget.
#[derive(Debug, Default)]
pub struct Root {
    tick_counter: TickCounter,
}

impl Root {
    /// Handle a key event. Returns `Some(event)` if the event was not handled.
    pub fn handle_key_event(&mut self, event: KeyEvent) -> Option<KeyEvent> {
        let unhandled = self.tick_counter.handle_key_event(event);
        // Add any key events that are handled by root widget here.
        unhandled
    }

    /// Handles the tick event. This is a good place to handle animations and other time-based events.
    pub fn tick(&mut self) {
        self.tick_counter.tick();
    }
}

impl Widget for &Root {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // This is where you add new widgets. See the following resources:
        // - https://ratatui.rs/how-to/widgets/
        // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
        // - https://github.com/ratatui-org/ratatui/tree/master/examples

        use Constraint::*;
        let [header, body, footer] = Layout::vertical([Length(1), Fill(1), Length(1)]).areas(area);

        // header
        let header_fg = tailwind::SLATE.c300;
        let header_bg = tailwind::AMBER.c900;
        let [left, right] = Layout::horizontal([Fill(1), Fill(1)]).areas(header);
        Line::from("Ratatui Template")
            .style((header_fg, header_bg, Modifier::BOLD))
            .render(left, buf);
        Line::from(format!("Mode: {}", self.tick_counter.mode()))
            .right_aligned()
            .style((header_fg, header_bg, Modifier::BOLD))
            .render(right, buf);

        self.tick_counter.render(body, buf);

        // footer
        Line::from("Quit: q, Reset: r, Pause: Space")
            .style((header_fg, header_bg, Modifier::BOLD))
            .render(footer, buf);
    }
}
