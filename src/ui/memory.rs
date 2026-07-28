use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Sparkline},
    Frame,
};

pub fn render_memory_tab(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // RAM Gauge
            Constraint::Length(3), // RAM Sparkline History Chart
            Constraint::Length(4), // Swap Gauge
            Constraint::Min(4),    // Detailed Breakdown
        ])
        .split(area);

    let ram = &app.metrics.memory;
    let ram_percent = ram.ram_usage_percent().clamp(0.0, 100.0);
    let swap_percent = ram.swap_usage_percent().clamp(0.0, 100.0);

    // RAM Gauge
    let ram_gauge = Gauge::default()
        .block(Block::default().title(format!(
            " Physical RAM: {:.2} GB / {:.2} GB ({:.1}%) ",
            ram.used_mem_bytes as f64 / 1e9,
            ram.total_mem_bytes as f64 / 1e9,
            ram_percent
        )).borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::LightCyan).bg(Color::Black))
        .percent(ram_percent as u16);
    f.render_widget(ram_gauge, chunks[0]);

    // RAM Sparkline History
    let ram_data: Vec<u64> = app.metrics.ram_history.iter().copied().collect();
    let sparkline = Sparkline::default()
        .block(Block::default().title(" Real-time RAM History (Last 60s) ").borders(Borders::ALL))
        .data(&ram_data)
        .max(100)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(sparkline, chunks[1]);

    // Swap Gauge
    let swap_gauge = Gauge::default()
        .block(Block::default().title(format!(
            " Swap Space: {:.2} GB / {:.2} GB ({:.1}%) ",
            ram.used_swap_bytes as f64 / 1e9,
            ram.total_swap_bytes as f64 / 1e9,
            swap_percent
        )).borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Magenta).bg(Color::Black))
        .percent(swap_percent as u16);
    f.render_widget(swap_gauge, chunks[2]);

    // Detailed Breakdown
    let details = Paragraph::new(format!(
        "Memory Breakdown:\n\n• Total Memory: {:.2} GB ({}) bytes\n• Used Memory:  {:.2} GB ({}) bytes\n• Free Memory:  {:.2} GB ({}) bytes\n• Swap Total:   {:.2} GB\n• Swap Used:    {:.2} GB",
        ram.total_mem_bytes as f64 / 1e9, ram.total_mem_bytes,
        ram.used_mem_bytes as f64 / 1e9, ram.used_mem_bytes,
        ram.free_mem_bytes as f64 / 1e9, ram.free_mem_bytes,
        ram.total_swap_bytes as f64 / 1e9,
        ram.used_swap_bytes as f64 / 1e9,
    ))
    .block(Block::default().borders(Borders::ALL).title(" Memory Details "));
    f.render_widget(details, chunks[3]);
}
