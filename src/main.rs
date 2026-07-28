mod app;
mod event;
mod system;
mod ui;

use app::App;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup raw terminal mode and alternate screen
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create App state instance
    let mut app = App::new();
    let tick_rate = Duration::from_millis(500);

    // Main Event & Render Loop
    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &app))?;
        event::handle_events(&mut app, tick_rate)?;
        app.tick();
    }

    // Clean up terminal on exit
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
