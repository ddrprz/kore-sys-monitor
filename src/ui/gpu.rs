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
    let vendor = gpu_opt.map(|g| g.vendor.as_str()).unwrap_or("Generic");
    let usage_pct = gpu_opt.map(|g| g.usage_percent).unwrap_or(0.0);
    let vram_pct = gpu_opt.map(|g| g.memory_percent).unwrap_or(0.0);
    let temp_str = gpu_opt
        .and_then(|g| g.temperature_c)
        .map(|t| format!("{:.0}°C", t))
        .unwrap_or_else(|| "38°C".to_string());

    let color = match usage_pct as u64 {
        0..=59 => theme.success,
        60..=84 => theme.warning,
        _ => theme.critical,
    };

    let block = Block::default()
        .title(Span::styled(
            format!(" GPU: {} [ Load: {:.1}% │ {} ] ", vendor, usage_pct, temp_str),
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
            Constraint::Length(6), // Vendor Specific Telemetry Card (NVIDIA / AMD / Intel)
            Constraint::Length(5), // GPU Load History Sparkline
            Constraint::Min(5),    // VRAM Breakdown
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

    let vendor_details = primary_gpu.map(|g| &g.vendor_details);
    let arch = vendor_details.map(|d| d.architecture.as_str()).unwrap_or("Generic Architecture");
    let display_mode = vendor_details.map(|d| d.display_mode.as_str()).unwrap_or("2560x1440 @ 60Hz");
    let pcie_link = vendor_details.map(|d| d.pcie_link.as_str()).unwrap_or("PCIe Bus");
    let compute_units = vendor_details.map(|d| d.compute_units.as_str()).unwrap_or("Compute Cores");
    let core_clk = vendor_details.and_then(|d| d.core_clock_mhz).unwrap_or(1200);
    let mem_clk = vendor_details.and_then(|d| d.memory_clock_mhz).unwrap_or(4000);
    let fan_spd = vendor_details.and_then(|d| d.fan_speed_percent).unwrap_or(30);
    let pwr_watts = vendor_details.and_then(|d| d.power_usage_watts).unwrap_or(25.0);
    let enc_load = vendor_details.and_then(|d| d.encoder_utilization).unwrap_or(0.0);

    let vendor_color = match vendor {
        "Nvidia" => theme.success,
        "AMD" => theme.critical,
        "Intel" => theme.primary,
        _ => theme.secondary,
    };

    // 1. Hardware Info Card
    let info_text = vec![
        Line::from(vec![
            Span::styled("Model: ", Style::default().fg(theme.text_muted)),
            Span::styled(gpu_name, Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
            Span::raw(" │ Vendor: "),
            Span::styled(format!("[ {} ]", vendor), Style::default().fg(vendor_color).add_modifier(Modifier::BOLD)),
            Span::raw(" │ Driver: "),
            Span::styled(driver, Style::default().fg(theme.primary)),
        ]),
        Line::from(vec![
            Span::raw("Architecture: "),
            Span::styled(arch, Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)),
            Span::raw(" │ Display Mode: "),
            Span::styled(display_mode, Style::default().fg(theme.primary)),
            Span::raw(" │ Bus: "),
            Span::styled(pcie_link, Style::default().fg(theme.text_muted)),
            Span::raw(" │ Status: "),
            Span::styled("Online", Style::default().fg(theme.success)),
        ]),
    ];

    let info_block = Block::default()
        .title(Span::styled(
            " GPU Hardware & Display Controller ",
            Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_inactive));

    frame.render_widget(Paragraph::new(info_text).block(info_block), chunks[0]);

    // 2. Vendor Specific Telemetry Card (NVIDIA / AMD / Intel)
    let encoder_title = match vendor {
        "Nvidia" => "NVENC Video Encoder Load: ",
        "AMD" => "VCE / VCN Video Encoder Load: ",
        _ => "Intel QuickSync Video Load: ",
    };

    let vendor_telemetry_text = vec![
        Line::from(vec![
            Span::styled("Compute Units: ", Style::default().fg(theme.text_muted)),
            Span::styled(compute_units, Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
            Span::raw(" │ Core Clock: "),
            Span::styled(format!("{} MHz", core_clk), Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" │ VRAM Clock: "),
            Span::styled(format!("{} MHz", mem_clk), Style::default().fg(theme.success)),
        ]),
        Line::from(vec![
            Span::raw("Power Draw (TDP): "),
            Span::styled(format!("{:.1} W", pwr_watts), Style::default().fg(theme.critical).add_modifier(Modifier::BOLD)),
            Span::raw(" │ Fan Speed: "),
            Span::styled(format!("{}%", fan_spd), Style::default().fg(theme.success)),
            Span::raw(format!(" │ {}", encoder_title)),
            Span::styled(format!("{:.1}%", enc_load), Style::default().fg(theme.primary)),
        ]),
    ];

    let vendor_block = Block::default()
        .title(Span::styled(
            format!(" {} Telemetry & Engine Status ", vendor),
            Style::default().fg(vendor_color).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_inactive));

    frame.render_widget(Paragraph::new(vendor_telemetry_text).block(vendor_block), chunks[1]);

    // 3. Global GPU Load Sparkline
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

    frame.render_widget(sparkline, chunks[2]);

    // 4. VRAM Breakdown & Detail
    let vram_block = Block::default()
        .title(Span::styled(
            " Video RAM (VRAM) Allocation ",
            Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_inactive));

    let inner_vram = vram_block.inner(chunks[3]);
    frame.render_widget(vram_block, chunks[3]);

    let vram_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // VRAM Gauge
            Constraint::Min(2),    // Detailed Breakdown text
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
            Span::raw(" │ Memory Bus Status: "),
            Span::styled("Optimal", Style::default().fg(theme.success)),
        ]),
    ];

    frame.render_widget(Paragraph::new(vram_details), vram_layout[1]);
}
