use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

pub fn render_disks_tab(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5)])
        .split(area);

    let header_cells = [
        "Disk",
        "Mount Point",
        "File System",
        "Used Space",
        "Total Space",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.metrics.disks.iter().map(|disk| {
        let total_gb = disk.total_bytes as f64 / 1e9;
        let avail_gb = disk.available_bytes as f64 / 1e9;
        let used_gb = total_gb - avail_gb;
        let used_percent = if total_gb > 0.0 {
            (used_gb / total_gb * 100.0) as u16
        } else {
            0
        };

        let cells = vec![
            Cell::from(disk.name.clone()),
            Cell::from(disk.mount_point.clone()),
            Cell::from(disk.file_system.clone()),
            Cell::from(format!("{:.2} GB ({}%)", used_gb, used_percent)),
            Cell::from(format!("{:.2} GB", total_gb)),
        ];
        Row::new(cells)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(15),
            Constraint::Min(20),
            Constraint::Length(12),
            Constraint::Length(20),
            Constraint::Length(15),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Storage Drives ({}) ", app.metrics.disks.len())),
    );

    f.render_widget(table, chunks[0]);
}
