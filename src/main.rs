/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Terminal user interface.
pub mod tui;

///User interface controller
pub mod ui;

pub mod update;

///Heightmap generator
pub mod heightmapgen;

///Trajectory generator
pub mod pathgen;

use anyhow::Result;
use app::App;
use event::{Event, EventHandler};
use ratatui::{Terminal, backend::CrosstermBackend};
use tui::Tui;
use crate::{pathgen::DetectionMode, update::update_core::update};

fn main() -> Result<()> {
    // Create an application.
    let mut app = App::new();

    //Setup the starting settings
    app.path_gen_info.detect_info = DetectionMode::SIMPLE.get_default_settings();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
