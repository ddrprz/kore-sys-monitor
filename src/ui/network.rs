use crate::app::App;
use crate::system::{ConnectionMedium, SpeedTestState};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Sparkline},
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

pub fn render_detail(app: &App, frame: &mut Frame, area: Rect) {
    render(app, frame, area);
}

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let theme = &app.theme;

    let outer_block = Block::default()
        .title(Span::styled(
            " Network & Connectivity ",
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

    // Determine layout: Connection Summary (WiFi/Cable, SSID, Gateway), Speed Test, and Traffic Sparklines
    let summary_height = if inner.height >= 24 { 6 } else if inner.height >= 18 { 5 } else { 4 };
    let speed_test_height = if inner.height >= 22 { 7 } else if inner.height >= 14 { 6 } else { 4 };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(summary_height),   // Simplified Connection Summary (WiFi/Cable, Network Name, Gateway)
            Constraint::Length(speed_test_height), // Speed Test Interactive Panel
            Constraint::Min(4),                   // Live Traffic Sparklines (RX / TX)
        ])
        .split(inner);

    // 1. Connection Summary Section (Medium, SSID, Gateway)
    render_connection_summary(app, frame, chunks[0]);

    // 2. Speed Test Section
    render_speed_test_panel(app, frame, chunks[1]);

    // 3. Traffic Sparklines
    render_sparklines(app, frame, chunks[2]);
}

fn render_connection_summary(app: &App, frame: &mut Frame, area: Rect) {
    let theme = &app.theme;

    let medium_str = app.metrics.primary_medium.as_str();

    let (medium_icon, medium_color) = match app.metrics.primary_medium {
        ConnectionMedium::WiFi => ("📶", theme.success),
        ConnectionMedium::Cable => ("🔌", theme.primary),
        ConnectionMedium::Virtual => ("🖧", theme.warning),
        ConnectionMedium::Disconnected => ("❌", theme.critical),
    };

    let network_name = &app.metrics.primary_network_name;
    let gateway = &app.metrics.primary_gateway;
    let local_ip = &app.metrics.primary_ip;

    let is_wide = area.width >= 80;

    if is_wide && area.height >= 4 {
        // 3 Cards: Tipo de Conexión, Red / SSID, Gateway
        let cards = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(area);

        // Card 1: Medio / Tipo de Conexión
        let p_medium = Paragraph::new(vec![
            Line::from(Span::styled(" TIPO DE CONEXIÓN", Style::default().fg(theme.text_muted))),
            Line::from(vec![
                Span::styled(format!(" {} ", medium_icon), Style::default().fg(medium_color)),
                Span::styled(medium_str, Style::default().fg(medium_color).add_modifier(Modifier::BOLD)),
            ]),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.border_inactive)),
        );

        // Card 2: Nombre de la Red (SSID)
        let p_network = Paragraph::new(vec![
            Line::from(Span::styled(" NOMBRE DE LA RED (SSID)", Style::default().fg(theme.text_muted))),
            Line::from(vec![
                Span::styled(" 🌐 ", Style::default().fg(theme.primary)),
                Span::styled(network_name, Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
            ]),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.border_inactive)),
        );

        // Card 3: Gateway (Puerta de Enlace)
        let p_gateway = Paragraph::new(vec![
            Line::from(Span::styled(" GATEWAY (PUERTA DE ENLACE)", Style::default().fg(theme.text_muted))),
            Line::from(vec![
                Span::styled(" 🚪 ", Style::default().fg(theme.warning)),
                Span::styled(gateway, Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            ]),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.border_inactive)),
        );

        frame.render_widget(p_medium, cards[0]);
        frame.render_widget(p_network, cards[1]);
        frame.render_widget(p_gateway, cards[2]);
    } else {
        // Compact multi-line summary layout
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme.border_inactive));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let lines = vec![
            Line::from(vec![
                Span::styled(format!(" {} Conexión: ", medium_icon), Style::default().fg(theme.text_muted)),
                Span::styled(medium_str, Style::default().fg(medium_color).add_modifier(Modifier::BOLD)),
                Span::styled(" │ Red: ", Style::default().fg(theme.text_muted)),
                Span::styled(network_name, Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled(" 🚪 Gateway: ", Style::default().fg(theme.text_muted)),
                Span::styled(gateway, Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
                Span::styled(" │ IP Local: ", Style::default().fg(theme.text_muted)),
                Span::styled(local_ip, Style::default().fg(theme.secondary)),
            ]),
        ];

        frame.render_widget(Paragraph::new(lines), inner);
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
            " Speed Test de Red [e] ",
            Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
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

    if area.width >= 90 && inner.height >= 3 {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(2)])
            .split(inner);

        // Status line
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
        // Compact format
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
                Span::styled("  │  ↓: ", Style::default().fg(theme.text_muted)),
                Span::styled(dl_str, Style::default().fg(theme.success).add_modifier(Modifier::BOLD)),
                Span::styled("  │  ↑: ", Style::default().fg(theme.text_muted)),
                Span::styled(ul_str, Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)),
            ]),
        ];
        frame.render_widget(Paragraph::new(text), inner);
    }
}

fn render_sparklines(app: &App, frame: &mut Frame, area: Rect) {
    let theme = &app.theme;
    let is_wide_screen = area.width >= 100;

    let rx_data: Vec<u64> = app.metrics.rx_history.iter().copied().collect();
    let tx_data: Vec<u64> = app.metrics.tx_history.iter().copied().collect();

    let rx_block = Block::default()
        .title(Span::styled(
            format!(
                " Tráfico Descarga (RX): {:.2} KB/s │ Total: {} ",
                app.metrics.rx_rate_kbs,
                format_net_bytes(app.metrics.total_rx_bytes)
            ),
            Style::default().fg(theme.success).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_inactive));

    let rx_sparkline = Sparkline::default()
        .block(rx_block)
        .data(&rx_data)
        .style(Style::default().fg(theme.success));

    let tx_block = Block::default()
        .title(Span::styled(
            format!(
                " Tráfico Subida (TX): {:.2} KB/s │ Total: {} ",
                app.metrics.tx_rate_kbs,
                format_net_bytes(app.metrics.total_tx_bytes)
            ),
            Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_inactive));

    let tx_sparkline = Sparkline::default()
        .block(tx_block)
        .data(&tx_data)
        .style(Style::default().fg(theme.secondary));

    if is_wide_screen {
        let spark_cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        frame.render_widget(rx_sparkline, spark_cols[0]);
        frame.render_widget(tx_sparkline, spark_cols[1]);
    } else {
        let spark_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        frame.render_widget(rx_sparkline, spark_rows[0]);
        frame.render_widget(tx_sparkline, spark_rows[1]);
    }
}
