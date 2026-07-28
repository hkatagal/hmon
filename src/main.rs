use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Terminal,
};
use sysinfo::System;
use std::{io, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create system stats monitor
    let mut sys = System::new_all();

    let res = run_app(&mut terminal, &mut sys);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    sys: &mut System,
) -> io::Result<()> {
    loop {
        sys.refresh_all();

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3), // Header
                        Constraint::Length(4), // CPU Gauge
                        Constraint::Length(4), // Memory Gauge
                        Constraint::Min(5),    // Quick Summary & Controls
                    ]
                    .as_ref(),
                )
                .split(f.area());

            // 1. Header Block
            let header = Paragraph::new("⚡ hmon v0.1.0 — Harish's System Monitor (Press 'q' to quit)")
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL).title(" Overview "));
            f.render_widget(header, chunks[0]);

            // 2. CPU Usage Gauge
            let global_cpu = sys.global_cpu_usage();
            let cpu_gauge = Gauge::default()
                .block(Block::default().title(format!(" CPU Usage: {:.1}% ", global_cpu)).borders(Borders::ALL))
                .gauge_style(
                    Style::default()
                        .fg(if global_cpu > 80.0 { Color::Red } else { Color::Green })
                        .bg(Color::Black),
                )
                .percent(global_cpu as u16);
            f.render_widget(cpu_gauge, chunks[1]);

            // 3. Memory Usage Gauge
            let total_mem = sys.total_memory() as f64;
            let used_mem = sys.used_memory() as f64;
            let mem_percent = if total_mem > 0.0 { (used_mem / total_mem * 100.0) as u16 } else { 0 };
            let mem_gauge = Gauge::default()
                .block(Block::default().title(format!(" RAM Usage: {:.2} GB / {:.2} GB ", used_mem / 1e9, total_mem / 1e9)).borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Yellow).bg(Color::Black))
                .percent(mem_percent);
            f.render_widget(mem_gauge, chunks[2]);

            // 4. System & Process Summary
            let info_text = vec![
                ListItem::new(format!("🖥️  Host Name: {}", System::host_name().unwrap_or_else(|| "Unknown".into()))),
                ListItem::new(format!("🐧 OS System: {} {}", System::name().unwrap_or_else(|| "OS".into()), System::os_version().unwrap_or_default())),
                ListItem::new(format!("⚙️  Total Processes: {}", sys.processes().len())),
                ListItem::new(format!("⚡ CPU Cores: {}", sys.cpus().len())),
                ListItem::new("💡 Controls: Press 'q' or 'Esc' to exit. More features coming soon!"),
            ];
            let list = List::new(info_text).block(
                Block::default()
                    .title(" System Summary ")
                    .borders(Borders::ALL),
            );
            f.render_widget(list, chunks[3]);
        })?;

        // Handle key events with 500ms refresh rate
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    return Ok(());
                }
            }
        }
    }
}
