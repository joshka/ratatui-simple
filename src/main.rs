pub mod app;
pub mod errors;
pub mod events;
pub mod tui;

use color_eyre::Result;
use {app::App, events::EventHandler};

fn main() -> Result<()> {
    errors::init()?;
    let terminal = &mut tui::init()?;
    let events = &mut EventHandler::new();
    App::new().run(terminal, events)?;
    Ok(())
}
