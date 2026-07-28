use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

pub fn render_processes_tab(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(area);

    // Search & Filter Status
    let filter_text = if app.is_searching {
        format!("🔍 Search Filter (type to filter, Enter/Esc to lock): {}_", app.search_query)
    } else if !app.search_query.is_empty() {
        format!("🔍 Filter Active: '{}' (Press '/' to edit, Esc to clear) | Sort: {:?}", app.search_query, app.sort_key)
    } else {
        format!("⚙️ Total Processes: {} | Sort: {:?} (Press 's' to cycle sort, '/' to search, 'k' to kill)", app.metrics.processes.len(), app.sort_key)
    };

    let search_bar = Paragraph::new(filter_text)
        .style(Style::default().fg(if app.is_searching { Color::Yellow } else { Color::Cyan }))
        .block(Block::default().borders(Borders::ALL).title(" Process Manager Controls "));
    f.render_widget(search_bar, chunks[0]);

    // Process Table
    let header_cells = ["PID", "Name", "CPU %", "Memory (MB)", "Status"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.metrics.processes.iter().enumerate().map(|(idx, proc)| {
        let is_selected = idx == app.selected_proc_index;
        let style = if is_selected {
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let mem_mb = proc.memory_bytes as f64 / 1_000_000.0;
        let cells = vec![
            Cell::from(proc.pid.to_string()),
            Cell::from(proc.name.clone()),
            Cell::from(format!("{:.1}%", proc.cpu_usage)),
            Cell::from(format!("{:.1} MB", mem_mb)),
            Cell::from(proc.status.clone()),
        ];
        Row::new(cells).style(style)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(10), // PID
            Constraint::Min(20),   // Name
            Constraint::Length(10), // CPU %
            Constraint::Length(15), // Memory
            Constraint::Length(15), // Status
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(format!(" Processes ({}) ", app.metrics.processes.len())));

    f.render_widget(table, chunks[1]);
}
