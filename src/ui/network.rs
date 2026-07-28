use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

pub fn render_network_tab(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5)])
        .split(area);

    let header_cells = ["Interface", "MAC Address", "Download Rate", "Upload Rate", "Total Received", "Total Transmitted"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.metrics.network.interfaces.iter().map(|iface| {
        let rx_kb = iface.rx_bytes as f64 / 1024.0;
        let tx_kb = iface.tx_bytes as f64 / 1024.0;
        let tot_rx_mb = iface.total_rx_bytes as f64 / 1e6;
        let tot_tx_mb = iface.total_tx_bytes as f64 / 1e6;

        let cells = vec![
            Cell::from(iface.name.clone()),
            Cell::from(iface.mac_address.clone()),
            Cell::from(format!("{:.1} KB/s", rx_kb)),
            Cell::from(format!("{:.1} KB/s", tx_kb)),
            Cell::from(format!("{:.2} MB", tot_rx_mb)),
            Cell::from(format!("{:.2} MB", tot_tx_mb)),
        ];
        Row::new(cells)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(15),
            Constraint::Length(20),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(18),
            Constraint::Length(18),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(format!(
        " Network Interfaces ({}) | Total Rx: {:.1} KB/s | Total Tx: {:.1} KB/s ",
        app.metrics.network.interfaces.len(),
        app.metrics.network.total_rx_rate_sec as f64 / 1024.0,
        app.metrics.network.total_tx_rate_sec as f64 / 1024.0
    )));

    f.render_widget(table, chunks[0]);
}
