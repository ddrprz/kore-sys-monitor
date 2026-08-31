use crate::app::App;
use crate::system::SpeedTestState;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Sparkline, Table},
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
            " Network, Connected Adapters & Speed Test ",
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

    // Determine layout: Adapters Table, Sparklines, and Speed Test section
    let sparkline_height = if inner.height >= 22 { 4 } else { 3 };
    let speed_test_height = if inner.height >= 18 { 6 } else if inner.height >= 12 { 5 } else { 4 };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(4),                   // Adapters Table
            Constraint::Length(speed_test_height), // Speed Test Panel (Ordered before RX/TX)
            Constraint::Length(sparkline_height),  // Traffic Sparklines (RX/TX)
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
                Constraint::Percentage(22),
                Constraint::Percentage(11),
                Constraint::Percentage(13),
                Constraint::Percentage(12),
                Constraint::Percentage(14),
                Constraint::Percentage(9),
                Constraint::Percentage(9),
                Constraint::Percentage(10),
            ],
        )
        .header(header)
        .block(
            Block::default()
                .title(Span::styled(" Network Adapters & Interfaces ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)))
                .borders(Borders::NONE),
        )
    } else {
        // Compact 2-Line Row Layout for standard screens
        let header_cells = [
            "Interface / Model Details",
            "Status",
            "Network Config (IP / GW / DNS)",
            "Traffic (Rate & Total)",
        ]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));
        let header = Row::new(header_cells).height(1);

        let rows = app.metrics.network_interfaces.iter().take(6).map(|iface| {
            let is_connected = iface.is_up || (iface.ip_address != "-" && !iface.ip_address.is_empty());
            let (status_symbol, status_text, status_color) = if is_connected {
                ("●", " CONNECTED", theme.success)
            } else {
                ("○", " IDLE", theme.text_muted)
            };

            let ip_display = if iface.ip_address.is_empty() { "-" } else { &iface.ip_address };
            let gw_display = if iface.gateway.is_empty() { "-" } else { &iface.gateway };
            let dns_display = if iface.dns_servers.is_empty() { "-" } else { &iface.dns_servers };

            let col0 = Text::from(vec![
                Line::from(Span::styled(iface.model.clone(), Style::default().fg(theme.primary).add_modifier(Modifier::BOLD))),
                Line::from(Span::styled(format!("Dev: {}", iface.name), Style::default().fg(theme.text_muted))),
            ]);

            let col1 = Text::from(vec![
                Line::from(Span::styled(format!("{}{}", status_symbol, status_text), Style::default().fg(status_color).add_modifier(Modifier::BOLD))),
                Line::from(Span::styled(if is_connected { "Active Link" } else { "No Carrier" }, Style::default().fg(theme.text_muted))),
            ]);

            let col2 = Text::from(vec![
                Line::from(vec![
                    Span::styled("IP: ", Style::default().fg(theme.text_muted)),
                    Span::styled(ip_display, Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
                    Span::styled(" │ GW: ", Style::default().fg(theme.text_muted)),
                    Span::styled(gw_display, Style::default().fg(theme.warning)),
                ]),
                Line::from(vec![
                    Span::styled("DNS: ", Style::default().fg(theme.text_muted)),
                    Span::styled(dns_display, Style::default().fg(theme.secondary)),
                ]),
            ]);

            let col3 = Text::from(vec![
                Line::from(vec![
                    Span::styled(format!("↓ {:.1} KB/s", iface.rx_rate_kbs), Style::default().fg(theme.success).add_modifier(Modifier::BOLD)),
                    Span::styled(" │ ", Style::default().fg(theme.border_inactive)),
                    Span::styled(format!("↑ {:.1} KB/s", iface.tx_rate_kbs), Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(Span::styled(
                    format!("Total: ↓{}  ↑{}", format_net_bytes(iface.rx_bytes), format_net_bytes(iface.tx_bytes)),
                    Style::default().fg(theme.text_muted),
                )),
            ]);

            Row::new(vec![
                Cell::from(col0),
                Cell::from(col1),
                Cell::from(col2),
                Cell::from(col3),
            ])
            .height(2)
        });

        Table::new(
            rows,
            [
                Constraint::Percentage(28),
                Constraint::Percentage(14),
                Constraint::Percentage(34),
                Constraint::Percentage(24),
            ],
        )
        .header(header)
        .block(
            Block::default()
                .title(Span::styled(" Network Adapters & Interfaces ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)))
                .borders(Borders::NONE),
        )
    };

    frame.render_widget(adapters_table, chunks[0]);

    // 2. Render Speed Test Section directly above Traffic Sparklines
    render_speed_test_panel(app, frame, chunks[1]);

    // 3. Render Sparklines (Side-by-side on wide screens, stacked on narrow)
    let is_wide_screen = inner.width >= 100;
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
            .split(chunks[2]);

        frame.render_widget(rx_sparkline, spark_cols[0]);
        frame.render_widget(tx_sparkline, spark_cols[1]);
    } else {
        let spark_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[2]);

        frame.render_widget(rx_sparkline, spark_rows[0]);
        frame.render_widget(tx_sparkline, spark_rows[1]);
    }
}

pub fn render_speed_test_panel(app: &App, frame: &mut Frame, area: Rect) {
    let theme = &app.theme;
    let st = &app.speed_test;

    let (status_icon, status_label, status_color) = match &st.state {
        SpeedTestState::Idle => ("○", "LISTO / INACTIVO (Presiona [e] para iniciar)".to_string(), theme.primary),
        SpeedTestState::TestingPing => ("⏳", "Midiendo Latencia (Ping / RTT)...".to_string(), theme.warning),
        SpeedTestState::TestingDownload { progress_pct, current_mbps } => (
            "⏳",
            format!("Probando Descarga (↓ {:.1} Mbps - {}%)...", current_mbps, progress_pct),
            theme.success,
        ),
        SpeedTestState::TestingUpload { progress_pct, current_mbps } => (
            "⏳",
            format!("Probando Subida (↑ {:.1} Mbps - {}%)...", current_mbps, progress_pct),
            theme.secondary,
        ),
        SpeedTestState::Completed => {
            let ago = st.last_tested_secs_ago.map(|s| format!("hace {}s", s)).unwrap_or_else(|| "reciente".to_string());
            ("●", format!("COMPLETADO ({}) - Presiona [e] para repetir", ago), theme.success)
        }
        SpeedTestState::Failed(err) => ("✖", format!("ERROR ({}) - Presiona [e] para reintentar", err), theme.critical),
    };

    let block = Block::default()
        .title(Span::styled(
            " Network Speed Test [e] ",
            Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::TOP)
        .border_style(Style::default().fg(theme.border_inactive));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height < 2 {
        return;
    }

    let ping_str = st.ping_ms.map(|p| format!("{:.1} ms", p)).unwrap_or_else(|| "-- ms".to_string());
    let dl_str = st.download_mbps.map(|d| format!("{:.1} Mbps", d)).unwrap_or_else(|| "-- Mbps".to_string());
    let ul_str = st.upload_mbps.map(|u| format!("{:.1} Mbps", u)).unwrap_or_else(|| "-- Mbps".to_string());
    let server_str = format!("{} ({})", st.server_name, st.server_location);

    if area.width >= 95 && inner.height >= 3 {
        // Multi-card layout on wide views
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(2)])
            .split(inner);

        // Status line with Server prominent
        let status_line = Line::from(vec![
            Span::styled(format!(" {} ", status_icon), Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            Span::styled("Estado: ", Style::default().fg(theme.text_muted)),
            Span::styled(status_label, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            Span::styled(" │ Servidor: ", Style::default().fg(theme.text_muted)),
            Span::styled(server_str, Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        ]);
        frame.render_widget(Paragraph::new(status_line), rows[0]);

        // 3 Cards: Ping, Download, Upload
        let cards = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(rows[1]);

        let card_ping = Paragraph::new(vec![
            Line::from(Span::styled(" LATENCIA / PING", Style::default().fg(theme.text_muted))),
            Line::from(Span::styled(format!(" ⚡ {}", ping_str), Style::default().fg(theme.warning).add_modifier(Modifier::BOLD))),
        ])
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(theme.border_inactive)));

        let card_dl = Paragraph::new(vec![
            Line::from(Span::styled(" VELOCIDAD DE BAJADA", Style::default().fg(theme.text_muted))),
            Line::from(Span::styled(format!(" ↓ {}", dl_str), Style::default().fg(theme.success).add_modifier(Modifier::BOLD))),
        ])
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(theme.border_inactive)));

        let card_ul = Paragraph::new(vec![
            Line::from(Span::styled(" VELOCIDAD DE SUBIDA", Style::default().fg(theme.text_muted))),
            Line::from(Span::styled(format!(" ↑ {}", ul_str), Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD))),
        ])
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(theme.border_inactive)));

        frame.render_widget(card_ping, cards[0]);
        frame.render_widget(card_dl, cards[1]);
        frame.render_widget(card_ul, cards[2]);
    } else {
        // Multi-line clean compact layout (No clipping)
        let text = vec![
            Line::from(vec![
                Span::styled(format!(" {} ", status_icon), Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
                Span::styled(status_label, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled(" 📍 Servidor: ", Style::default().fg(theme.text_muted)),
                Span::styled(server_str, Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled(" ⚡ Ping: ", Style::default().fg(theme.text_muted)),
                Span::styled(ping_str, Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
                Span::styled("  │  ↓ Bajada: ", Style::default().fg(theme.text_muted)),
                Span::styled(dl_str, Style::default().fg(theme.success).add_modifier(Modifier::BOLD)),
                Span::styled("  │  ↑ Subida: ", Style::default().fg(theme.text_muted)),
                Span::styled(ul_str, Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)),
            ]),
        ];
        frame.render_widget(Paragraph::new(text), inner);
    }
}
