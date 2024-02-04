# Ratatui Simple App

This is a WIP simple template app for [Ratatui]

Design choices:

- License is MIT OR Apache-2.0 (both included)
- TODO reference
- Uses color_eyre, crossterm
- Main is simple
  - install error hooks
  - creates terminal
  - creates event handler
  - App::new()::run(terminal, events)
- Error hooks
  - init() - color_eyre error hook and panic hook
- App
  - has a simple tick counter, header and footer
  - run()
    - loop controlled by app::mode
    - calls self.draw and self.handle_events
  - draw()
    - just renders the app as a widget
  - handle_events()
    - calls events.next()
    - uses conversion functions to convert from events to Action (`From<KeyEvent> for Action`)
    - calls methods on app based on the action `Event::Tick => self.tick()`
  - impl Widget for App
    - uses layout and tailwind colors
    - header, body, footer
    - has a body widget to show decomposition
- Tui module
  - TerminalGuard impls `Deref<Terminal>`, `DerefMut<Terminal>`, `Drop`
  - `fn tui::init() -> TerminalGuard` (and `restore` is there if needed, but shouldn't need to generally)
- Events
  - mpsc::channel
  - two threads
    - tick (send Event::Tick every 100ms)
    - crossterm_events (gets the relevant events and sends them)
    - todo handle stopping the event thread (e.g. for running an editor)
  
[Ratatui]: https://ratatui.rs
