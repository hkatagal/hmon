pub mod header;
pub mod cpu;
pub mod memory;
pub mod processes;
pub mod disks;
pub mod footer;

use crate::app::{App, Tab};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(5),    // Main Content View
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // 1. Render Header
    header::render_header(f, app, chunks[0]);

    // 2. Render Active Tab View
    match app.active_tab {
        Tab::Overview | Tab::Processes => processes::render_processes_tab(f, app, chunks[1]),
        Tab::Cpu => cpu::render_cpu_tab(f, app, chunks[1]),
        Tab::Memory => memory::render_memory_tab(f, app, chunks[1]),
        Tab::Disks => disks::render_disks_tab(f, app, chunks[1]),
    }

    // 3. Render Footer
    footer::render_footer(f, app, chunks[2]);
}
