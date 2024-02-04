use std::{
    io::{stdout, Stdout},
    ops::{Deref, DerefMut},
};

use color_eyre::Result;
use crossterm::{execute, terminal::*};
use ratatui::prelude::*;

/// A guard wrapper around Terminal that restores the terminal properties when dropped.
///
/// This ensures that the terminal does not get stuck in raw mode or the alternate screen if the
/// application crashes.
#[derive(Debug)]
pub struct TerminalGuard {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

/// Pass-through implementations for the terminal.
impl Deref for TerminalGuard {
    type Target = Terminal<CrosstermBackend<Stdout>>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

/// Pass-through implementations for the terminal
impl DerefMut for TerminalGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

/// Restore the terminal when the guard is dropped.
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        restore().unwrap();
    }
}

/// Initialize the terminal interface.
///
/// Returns a [`TerminalGuard`] that restores the terminal to its original state when dropped.
///
/// This enables [raw mode] and enters the [alternate screen].
///
/// [raw mode]: https://ratatui.rs/concepts/backends/raw-mode/
/// [alternate screen]: https://ratatui.rs/concepts/backends/alternate-screen/
pub fn init() -> Result<TerminalGuard> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(TerminalGuard { terminal })
}

/// Restore the terminal interface.
///
/// This disables [raw mode] and leaves the [alternate screen].
pub fn restore() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
