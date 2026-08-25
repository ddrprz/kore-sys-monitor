use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Row, Sparkline, Table},
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
    let theme = &app.theme;

    let outer_block = Block::default()
        .title(Span::styled(
            " Network & Adapters ",
            Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_inactive));

    let inner = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    if inner.height < 6 {
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(4),    // Adapters Table
            Constraint::Length(3), // RX Sparkline
            Constraint::Length(3), // TX Sparkline
        ])
        .split(inner);

    // 1. Network Adapters Table
    let header_cells = ["Adapter Model", "Status", "RX Speed", "TX Speed", "Total RX", "Total TX"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1);

    let rows = app.metrics.network_interfaces.iter().take(6).map(|iface| {
        let status_str = if iface.is_up { "UP" } else { "DOWN" };
        let status_color = if iface.is_up { theme.success } else { theme.text_muted };

        Row::new(vec![
            Cell::from(iface.model.clone()).style(Style::default().add_modifier(Modifier::BOLD).fg(theme.primary)),
            Cell::from(status_str).style(Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            Cell::from(format!("{:.1} KB/s", iface.rx_rate_kbs)).style(Style::default().fg(theme.success)),
            Cell::from(format!("{:.1} KB/s", iface.tx_rate_kbs)).style(Style::default().fg(theme.secondary)),
            Cell::from(format_net_bytes(iface.rx_bytes)),
            Cell::from(format_net_bytes(iface.tx_bytes)),
        ])
    });

    let adapters_table = Table::new(
        rows,
        [
            Constraint::Min(26),
            Constraint::Length(8),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(Span::styled(
                " Active Network Adapters ",
                Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(theme.border_inactive)),
    );

    frame.render_widget(adapters_table, chunks[0]);

    // 2. RX Sparkline
    let rx_data: Vec<u64> = app.metrics.rx_history.iter().copied().collect();
    let rx_block = Block::default().title(Span::styled(
        format!(
            " Global RX: {:.2} KB/s (Total: {}) ",
            app.metrics.rx_rate_kbs,
            format_net_bytes(app.metrics.total_rx_bytes)
        ),
        Style::default().fg(theme.success).add_modifier(Modifier::BOLD),
    ));

    let rx_sparkline = Sparkline::default()
        .block(rx_block)
        .data(&rx_data)
        .style(Style::default().fg(theme.success));

    frame.render_widget(rx_sparkline, chunks[1]);

    // 3. TX Sparkline
    let tx_data: Vec<u64> = app.metrics.tx_history.iter().copied().collect();
    let tx_block = Block::default().title(Span::styled(
        format!(
            " Global TX: {:.2} KB/s (Total: {}) ",
            app.metrics.tx_rate_kbs,
            format_net_bytes(app.metrics.total_tx_bytes)
        ),
        Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD),
    ));

    let tx_sparkline = Sparkline::default()
        .block(tx_block)
        .data(&tx_data)
        .style(Style::default().fg(theme.secondary));

    frame.render_widget(tx_sparkline, chunks[2]);
}
