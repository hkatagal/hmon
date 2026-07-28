use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let colors = app.config.theme.colors();

    let msg = app.status_message.as_deref().unwrap_or(
        "Nav: [1-6]/Tab/Left/Right | Proc: Up/Down | Sort: 's' | Search: '/' | Theme: 't' | Inspect: Enter | Quit: 'q'"
    );

    let footer = Paragraph::new(msg)
        .style(Style::default().fg(colors.accent))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Status & Controls "),
        );

    f.render_widget(footer, area);
}
