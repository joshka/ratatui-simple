pub mod app;
pub mod events;
pub mod hooks;
pub mod tui;

use color_eyre::Result;
use {app::App, events::EventHandler};

fn main() -> Result<()> {
    hooks::init()?;
    let terminal = &mut tui::init()?;
    let events = &mut EventHandler::new();
    App::new().run(terminal, events)?;
    Ok(())
}
