use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use yacht::app::{App, AppResult};
use yacht::event::{Event, EventHandler};
use yacht::handler::handle_key_events;
use yacht::init::initialize_activities;
use yacht::tui::Tui;

fn main() -> AppResult<()> {
    //init
    initialize_activities();

    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
