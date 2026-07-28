use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

pub fn render_cpu_tab(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(area);

    let brand_info = Paragraph::new(format!(
        "Processor: {} | Global CPU Usage: {:.1}% | Cores: {}",
        app.metrics.cpu.brand,
        app.metrics.cpu.global_usage,
        app.metrics.cpu.cores.len()
    ))
    .block(Block::default().borders(Borders::ALL).title(" CPU Info "));
    f.render_widget(brand_info, chunks[0]);

    // Render individual CPU core gauges
    let core_count = app.metrics.cpu.cores.len();
    if core_count > 0 {
        let constraints = vec![Constraint::Length(3); core_count.min(16)];
        let core_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(chunks[1]);

        for (i, core) in app.metrics.cpu.cores.iter().take(16).enumerate() {
            if i < core_chunks.len() {
                let usage = core.usage.clamp(0.0, 100.0);
                let color = if usage > 80.0 {
                    Color::Red
                } else if usage > 50.0 {
                    Color::Yellow
                } else {
                    Color::Green
                };

                let gauge = Gauge::default()
                    .block(Block::default().title(format!(" {} ({:.1}%) - {} MHz ", core.name, usage, core.frequency_mhz)))
                    .gauge_style(Style::default().fg(color).bg(Color::Black))
                    .percent(usage as u16);
                f.render_widget(gauge, core_chunks[i]);
            }
        }
    }
}
