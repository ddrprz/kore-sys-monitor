use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
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
            " Network & Connected Adapters ",
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

    // Determine layout: side-by-side sparklines on wide screens
    let is_wide_screen = inner.width >= 100;
    let sparkline_height = if inner.height >= 16 { 4 } else { 3 };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(4),
            Constraint::Length(sparkline_height),
        ])
        .split(inner);

    // 1. Connected Network Adapters Table (Responsive: 1-line vs 2-line auto-wrap)
    let is_wide_table = inner.width >= 120;

    let adapters_table = if is_wide_table {
        let header_cells = [
            "Interface / Adapter",
            "Status",
            "IP Address",
            "Gateway",
            "DNS Servers",
            "RX Rate",
            "TX Rate",
            "Total RX/TX",
        ]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));
        let header = Row::new(header_cells).height(1);

        let rows = app.metrics.network_interfaces.iter().take(8).map(|iface| {
            let is_connected = iface.is_up || (iface.ip_address != "-" && !iface.ip_address.is_empty());
            let (status_symbol, status_text, status_color) = if is_connected {
                ("●", " CONNECTED", theme.success)
            } else {
                ("○", " IDLE", theme.text_muted)
            };

            let ip_display = if iface.ip_address.is_empty() { "-" } else { &iface.ip_address };
            let gw_display = if iface.gateway.is_empty() { "-" } else { &iface.gateway };
            let dns_display = if iface.dns_servers.is_empty() { "-" } else { &iface.dns_servers };

            let total_str = format!("↓{} ↑{}", format_net_bytes(iface.rx_bytes), format_net_bytes(iface.tx_bytes));

            Row::new(vec![
                Cell::from(iface.model.clone()).style(Style::default().add_modifier(Modifier::BOLD).fg(theme.primary)),
                Cell::from(format!("{}{}", status_symbol, status_text)).style(Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
                Cell::from(ip_display.to_string()).style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
                Cell::from(gw_display.to_string()).style(Style::default().fg(theme.warning)),
                Cell::from(dns_display.to_string()).style(Style::default().fg(theme.secondary)),
                Cell::from(format!("↓ {:.1} KB/s", iface.rx_rate_kbs)).style(Style::default().fg(theme.success)),
                Cell::from(format!("↑ {:.1} KB/s", iface.tx_rate_kbs)).style(Style::default().fg(theme.secondary)),
                Cell::from(total_str).style(Style::default().fg(theme.text_muted)),
            ])
        });

        Table::new(
            rows,
            [
                Constraint::Min(22),
                Constraint::Length(14),
                Constraint::Length(17),
                Constraint::Length(15),
                Constraint::Length(18),
                Constraint::Length(13),
                Constraint::Length(13),
                Constraint::Length(18),
            ],
        )
        .header(header)
    } else {
        // Responsive 2-line mode to avoid cutting off info on narrow screens
        let header_cells = [
            "Adapter / Status",
            "IP & Gateway",
            "DNS Servers",
            "Current RX / TX",
            "Total RX / TX",
        ]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));
        let header = Row::new(header_cells).height(1);

        let rows = app.metrics.network_interfaces.iter().take(8).map(|iface| {
            let is_connected = iface.is_up || (iface.ip_address != "-" && !iface.ip_address.is_empty());
            let (status_symbol, status_text, status_color) = if is_connected {
                ("●", " CONNECTED", theme.success)
            } else {
                ("○", " IDLE", theme.text_muted)
            };

            let ip_display = if iface.ip_address.is_empty() { "-" } else { &iface.ip_address };
            let gw_display = if iface.gateway.is_empty() { "-" } else { &iface.gateway };
            let dns_display = if iface.dns_servers.is_empty() { "-" } else { &iface.dns_servers };

            let cell_adapter = Cell::from(Text::from(vec![
                Line::from(Span::styled(iface.model.clone(), Style::default().add_modifier(Modifier::BOLD).fg(theme.primary))),
                Line::from(Span::styled(format!("{}{}", status_symbol, status_text), Style::default().fg(status_color).add_modifier(Modifier::BOLD))),
            ]));

            let cell_ip_gw = Cell::from(Text::from(vec![
                Line::from(vec![
                    Span::styled("IP: ", Style::default().fg(theme.text_muted)),
                    Span::styled(ip_display.to_string(), Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("GW: ", Style::default().fg(theme.text_muted)),
                    Span::styled(gw_display.to_string(), Style::default().fg(theme.warning)),
                ]),
            ]));

            let cell_dns = Cell::from(Text::from(vec![
                Line::from(vec![
                    Span::styled("DNS: ", Style::default().fg(theme.text_muted)),
                    Span::styled(dns_display.to_string(), Style::default().fg(theme.secondary)),
                ]),
                Line::from(Span::styled(iface.name.clone(), Style::default().fg(theme.text_muted))),
            ]));

            let cell_rates = Cell::from(Text::from(vec![
                Line::from(Span::styled(format!("↓ {:.1} KB/s", iface.rx_rate_kbs), Style::default().fg(theme.success))),
                Line::from(Span::styled(format!("↑ {:.1} KB/s", iface.tx_rate_kbs), Style::default().fg(theme.secondary))),
            ]));

            let cell_total = Cell::from(Text::from(vec![
                Line::from(Span::styled(format!("↓ Tot: {}", format_net_bytes(iface.rx_bytes)), Style::default().fg(theme.text_muted))),
                Line::from(Span::styled(format!("↑ Tot: {}", format_net_bytes(iface.tx_bytes)), Style::default().fg(theme.text_muted))),
            ]));

            Row::new(vec![cell_adapter, cell_ip_gw, cell_dns, cell_rates, cell_total]).height(2)
        });

        Table::new(
            rows,
            [
                Constraint::Min(20),
                Constraint::Length(20),
                Constraint::Length(18),
                Constraint::Length(14),
                Constraint::Length(18),
            ],
        )
        .header(header)
    };

    let adapters_table = adapters_table.block(
        Block::default()
            .title(Span::styled(
                " Active Network Interfaces & Routes ",
                Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(theme.border_inactive)),
    );

    frame.render_widget(adapters_table, chunks[0]);

    // 2. Render Sparklines (Side-by-side on wide screens, stacked on narrow)
    let rx_data: Vec<u64> = app.metrics.rx_history.iter().copied().collect();
    let tx_data: Vec<u64> = app.metrics.tx_history.iter().copied().collect();

    let rx_block = Block::default().title(Span::styled(
        format!(
            " RX: {:.2} KB/s │ Total: {} ",
            app.metrics.rx_rate_kbs,
            format_net_bytes(app.metrics.total_rx_bytes)
        ),
        Style::default().fg(theme.success).add_modifier(Modifier::BOLD),
    ));

    let rx_sparkline = Sparkline::default()
        .block(rx_block)
        .data(&rx_data)
        .style(Style::default().fg(theme.success));

    let tx_block = Block::default().title(Span::styled(
        format!(
            " TX: {:.2} KB/s │ Total: {} ",
            app.metrics.tx_rate_kbs,
            format_net_bytes(app.metrics.total_tx_bytes)
        ),
        Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD),
    ));

    let tx_sparkline = Sparkline::default()
        .block(tx_block)
        .data(&tx_data)
        .style(Style::default().fg(theme.secondary));

    if is_wide_screen {
        let spark_cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        frame.render_widget(rx_sparkline, spark_cols[0]);
        frame.render_widget(tx_sparkline, spark_cols[1]);
    } else {
        let spark_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        frame.render_widget(rx_sparkline, spark_rows[0]);
        frame.render_widget(tx_sparkline, spark_rows[1]);
    }
}
