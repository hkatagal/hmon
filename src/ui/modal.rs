use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn render_proc_modal(f: &mut Frame, app: &App) {
    if !app.show_proc_modal {
        return;
    }

    if let Some(proc) = app.metrics.processes.get(app.selected_proc_index) {
        let area = centered_rect(60, 50, f.area());
        f.render_widget(Clear, area); // Clear background behind popup

        let colors = app.config.theme.colors();

        let detail_text = format!(
            "⚙️ Process Details Inspector\n\n\
             • Name: {}\n\
             • PID: {}\n\
             • CPU Usage: {:.2}%\n\
             • Memory Usage: {:.2} MB ({} bytes)\n\
             • Status: {}\n\n\
             💡 Press 'Enter' or 'Esc' to close this window.\n\
             💡 Press 'Shift + K' to terminate process.",
            proc.name,
            proc.pid,
            proc.cpu_usage,
            proc.memory_bytes as f64 / 1_000_000.0,
            proc.memory_bytes,
            proc.status
        );

        let popup = Paragraph::new(detail_text)
            .alignment(Alignment::Left)
            .style(Style::default().fg(colors.text).bg(Color::Black))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" Inspector: {} (PID: {}) ", proc.name, proc.pid))
                    .style(Style::default().fg(colors.primary).add_modifier(Modifier::BOLD)),
            );

        f.render_widget(popup, area);
    }
}

/// Helper function to create a centered Rect for modals
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
