use color_eyre::Result;
use ratatui_simple::{app::App, events::EventHandler, hooks, tui};

fn main() -> Result<()> {
    hooks::init()?;
    let terminal = &mut tui::init()?;
    let events = &mut EventHandler::new();
    App::new().run(terminal, events)?;
    Ok(())
}
