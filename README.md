# Ratatui Simple App

This is a WIP simple template app for [Ratatui]

## Todo

- [ ] Parameterize the template (authors, project name, desc, repo, username in copyright, license)
- [ ] Readme for end users
  - [ ] VHS script
- [ ] github workflows
- [ ] add <https://ratatui.rs/concepts/widgets/> doc
- [ ] handle pausing the event threads

## Design choices

- License is MIT OR Apache-2.0 (both included)
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
    - just renders root as a widget
  - handle_events()
    - calls events.next()
    - calls methods on app based on the action `Event::Tick => self.tick()`
- Root Widget
  - uses layout and tailwind colors
  - header, body, footer
  - has a tick counter widget to show decomposition and interaction
- Tui module
  - TerminalGuard impls `Deref<Terminal>`, `DerefMut<Terminal>`, `Drop`
  - `fn tui::init() -> TerminalGuard` (and `restore` is there if needed, but shouldn't need to generally)
- Events
  - mpsc::channel
  - two threads
    - tick (send Event::Tick every 100ms)
    - crossterm_events (gets the relevant events and sends them)
  
[Ratatui]: https://ratatui.rs
