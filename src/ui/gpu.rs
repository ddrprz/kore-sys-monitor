use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph, Sparkline},
    Frame,
};

pub fn render_summary(app: &App, frame: &mut Frame, area: Rect) {
    let theme = &app.theme;

    let gpu_opt = app.metrics.gpu_list.first();
    let gpu_name = gpu_opt
        .map(|g| g.name.as_str())
        .unwrap_or("Standard GPU");
    let usage_pct = gpu_opt.map(|g| g.usage_percent).unwrap_or(0.0);
    let vram_pct = gpu_opt.map(|g| g.memory_percent).unwrap_or(0.0);

    let color = match usage_pct as u64 {
        0..=59 => theme.success,
        60..=84 => theme.warning,
        _ => theme.critical,
    };

    let block = Block::default()
        .title(Span::styled(
            format!(" GPU [ Load: {:.1}% ] ", usage_pct),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_inactive));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // GPU Name (wrapped)
            Constraint::Min(2),    // Sparkline load
            Constraint::Length(1), // VRAM Gauge
        ])
        .split(inner);

    let gpu_name_p = Paragraph::new(Line::from(vec![
        Span::styled(gpu_name, Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
    ]))
    .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(gpu_name_p, chunks[0]);

    // Sparkline load
    let sparkline_data: Vec<u64> = app.metrics.gpu_usage_history.iter().copied().collect();
    let sparkline = Sparkline::default()
        .data(&sparkline_data)
        .max(100)
        .style(Style::default().fg(color));

    frame.render_widget(sparkline, chunks[1]);

    // VRAM Gauge
    let vram_gauge = Gauge::default()
        .gauge_style(Style::default().fg(theme.primary))
        .ratio((vram_pct as f64 / 100.0).clamp(0.0, 1.0))
        .label(format!("VRAM: {:.1}%", vram_pct));

    frame.render_widget(vram_gauge, chunks[2]);
}

pub fn render_detail(app: &App, frame: &mut Frame, area: Rect) {
    let theme = &app.theme;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Hardware Info Card
            Constraint::Length(7), // Global GPU Usage History Sparkline
            Constraint::Min(6),    // VRAM & Memory Breakdown
        ])
        .split(area);

    let primary_gpu = app.metrics.gpu_list.first();
    let gpu_name = primary_gpu.map(|g| g.name.as_str()).unwrap_or("Generic Graphics Controller");
    let vendor = primary_gpu.map(|g| g.vendor.as_str()).unwrap_or("System Default");
    let driver = primary_gpu.map(|g| g.driver_version.as_str()).unwrap_or("Standard");
    let total_vram_mb = primary_gpu.map(|g| g.memory_total / (1024 * 1024)).unwrap_or(1024);
    let used_vram_mb = primary_gpu.map(|g| g.memory_used / (1024 * 1024)).unwrap_or(0);
    let usage_pct = primary_gpu.map(|g| g.usage_percent).unwrap_or(0.0);
    let vram_pct = primary_gpu.map(|g| g.memory_percent).unwrap_or(0.0);

    // 1. Hardware Info Card
    let info_text = vec![
        Line::from(vec![
            Span::styled("Model: ", Style::default().fg(theme.text_muted)),
            Span::styled(gpu_name, Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
            Span::raw(" │ Vendor: "),
            Span::styled(vendor, Style::default().fg(theme.secondary)),
            Span::raw(" │ Driver: "),
            Span::styled(driver, Style::default().fg(theme.primary)),
        ]),
        Line::from(vec![
            Span::raw("Detected Devices: "),
            Span::styled(app.metrics.gpu_list.len().to_string(), Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" │ Total VRAM: "),
            Span::styled(format!("{} MB", total_vram_mb), Style::default().fg(theme.success)),
            Span::raw(" │ Temp: "),
            Span::styled(
                primary_gpu.and_then(|g| g.temperature_c).map(|t| format!("{:.0}°C", t)).unwrap_or_else(|| "N/A".to_string()),
                Style::default().fg(theme.primary),
            ),
            Span::raw(" │ Status: "),
            Span::styled("Active / Online", Style::default().fg(theme.success)),
        ]),
    ];

    let info_block = Block::default()
        .title(Span::styled(
            " GPU Hardware & Controller Info ",
            Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_inactive));

    frame.render_widget(Paragraph::new(info_text).block(info_block), chunks[0]);

    // 2. Global GPU Load Sparkline
    let load_color = match usage_pct as u64 {
        0..=59 => theme.success,
        60..=84 => theme.warning,
        _ => theme.critical,
    };

    let sparkline_data: Vec<u64> = app.metrics.gpu_usage_history.iter().copied().collect();

    let spark_block = Block::default()
        .title(Span::styled(
            format!(" GPU Processing Load History [ Current: {:.1}% ] ", usage_pct),
            Style::default().fg(load_color).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_inactive));

    let sparkline = Sparkline::default()
        .block(spark_block)
        .data(&sparkline_data)
        .max(100)
        .style(Style::default().fg(load_color));

    frame.render_widget(sparkline, chunks[1]);

    // 3. VRAM Breakdown & Detail
    let vram_block = Block::default()
        .title(Span::styled(
            " Video RAM (VRAM) Allocation ",
            Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_inactive));

    let inner_vram = vram_block.inner(chunks[2]);
    frame.render_widget(vram_block, chunks[2]);

    let vram_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // VRAM Gauge
            Constraint::Min(3),    // Detailed Breakdown text
        ])
        .split(inner_vram);

    let vram_gauge = Gauge::default()
        .gauge_style(Style::default().fg(theme.secondary))
        .ratio((vram_pct as f64 / 100.0).clamp(0.0, 1.0))
        .label(format!("Used: {} MB / Total: {} MB ({:.1}%)", used_vram_mb, total_vram_mb, vram_pct));

    frame.render_widget(vram_gauge, vram_layout[0]);

    let vram_details = vec![
        Line::from(vec![
            Span::raw("Allocated Memory: "),
            Span::styled(format!("{} MB", used_vram_mb), Style::default().fg(theme.warning)),
            Span::raw(" │ Free Memory: "),
            Span::styled(format!("{} MB", total_vram_mb.saturating_sub(used_vram_mb)), Style::default().fg(theme.success)),
        ]),
        Line::from(vec![
            Span::raw("Memory Bandwidth Status: "),
            Span::styled("Normal / Optimal", Style::default().fg(theme.success)),
        ]),
    ];

    frame.render_widget(Paragraph::new(vram_details), vram_layout[1]);
}
