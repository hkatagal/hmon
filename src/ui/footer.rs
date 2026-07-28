use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let msg = app.status_message.as_deref().unwrap_or(
        "Nav: [1-5]/Tab/Left/Right | Proc: Up/Down | Sort: 's' | Search: '/' | Kill: 'k' | Quit: 'q'"
    );

    let footer = Paragraph::new(msg)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title(" Status & Controls "));

    f.render_widget(footer, area);
}
