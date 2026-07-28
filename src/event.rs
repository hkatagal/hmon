use crate::app::{App, Tab};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::{io, time::Duration};

pub fn handle_events(app: &mut App, tick_rate: Duration) -> io::Result<()> {
    if event::poll(tick_rate)? {
        if let Event::Key(key) = event::read()? {
            // Handle Search Input Mode
            if app.is_searching {
                match key.code {
                    KeyCode::Enter | KeyCode::Esc => {
                        app.is_searching = false;
                    }
                    KeyCode::Backspace => {
                        app.search_query.pop();
                    }
                    KeyCode::Char(c) => {
                        app.search_query.push(c);
                    }
                    _ => {}
                }
                return Ok(());
            }

            // Normal Navigation Mode
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    if !app.search_query.is_empty() {
                        app.search_query.clear();
                    } else {
                        app.should_quit = true;
                    }
                }
                KeyCode::Tab => app.next_tab(),
                KeyCode::BackTab => app.previous_tab(),
                KeyCode::Right | KeyCode::Char('l') => app.next_tab(),
                KeyCode::Left | KeyCode::Char('h') => app.previous_tab(),

                // Direct Tab Jump Keys 1-5
                KeyCode::Char('1') => app.active_tab = Tab::Overview,
                KeyCode::Char('2') => app.active_tab = Tab::Cpu,
                KeyCode::Char('3') => app.active_tab = Tab::Memory,
                KeyCode::Char('4') => app.active_tab = Tab::Processes,
                KeyCode::Char('5') => app.active_tab = Tab::Disks,

                // Process Selection & Scrolling
                KeyCode::Down | KeyCode::Char('j') => app.next_process(),
                KeyCode::Up | KeyCode::Char('k') => app.previous_process(),

                // Sort & Search
                KeyCode::Char('s') => app.cycle_sort_key(),
                KeyCode::Char('/') => {
                    app.is_searching = true;
                    app.active_tab = Tab::Processes;
                }

                // Process Termination
                KeyCode::Char('K') if key.modifiers.contains(KeyModifiers::SHIFT) => {
                    app.kill_selected_process();
                }

                _ => {}
            }
        }
    }
    Ok(())
}
