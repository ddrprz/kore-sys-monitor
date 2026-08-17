use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Sparkline},
    Frame,
};

fn format_net_bytes(bytes: u64) -> String {
    if bytes > 1024 * 1024 * 1024 {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else if bytes > 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes > 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(Span::styled(
            " Network Bandwidth ",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .margin(1)
        .split(inner);

    // RX Sparkline
    let rx_data: Vec<u64> = app.metrics.rx_history.iter().copied().collect();
    let rx_block = Block::default().title(Span::styled(
        format!(
            " RX: {:.2} KB/s (Total: {}) ",
            app.metrics.rx_rate_kbs,
            format_net_bytes(app.metrics.total_rx_bytes)
        ),
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
    ));

    let rx_sparkline = Sparkline::default()
        .block(rx_block)
        .data(&rx_data)
        .style(Style::default().fg(Color::Green));

    frame.render_widget(rx_sparkline, chunks[0]);

    // TX Sparkline
    let tx_data: Vec<u64> = app.metrics.tx_history.iter().copied().collect();
    let tx_block = Block::default().title(Span::styled(
        format!(
            " TX: {:.2} KB/s (Total: {}) ",
            app.metrics.tx_rate_kbs,
            format_net_bytes(app.metrics.total_tx_bytes)
        ),
        Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
    ));

    let tx_sparkline = Sparkline::default()
        .block(tx_block)
        .data(&tx_data)
        .style(Style::default().fg(Color::Magenta));

    frame.render_widget(tx_sparkline, chunks[1]);
}
