use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
    Frame,
};

fn format_bytes(bytes: u64) -> String {
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{:.1} GB", gb)
}

fn format_file_size(bytes: u64) -> String {
    let kb = bytes as f64 / 1024.0;
    let mb = kb / 1024.0;
    let gb = mb / 1024.0;

    if gb >= 1.0 {
        format!("{:.2} GB", gb)
    } else if mb >= 1.0 {
        format!("{:.1} MB", mb)
    } else if kb >= 1.0 {
        format!("{:.0} KB", kb)
    } else {
        format!("{} B", bytes)
    }
}

fn format_gb_tb(gb: f64) -> String {
    if gb >= 1000.0 {
        format!("{:.1} TB", gb / 1024.0)
    } else {
        format!("{:.0} GB", gb)
    }
}

pub fn render_detail(app: &App, frame: &mut Frame, area: Rect) {
    render(app, frame, area);
}

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    if area.height >= 22 {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(34),
                Constraint::Percentage(32),
                Constraint::Percentage(34),
            ])
            .split(area);

        render_mounts_table(app, frame, chunks[0]);
        render_temp_files_table(app, frame, chunks[1]);
        render_smart_table(app, frame, chunks[2]);
    } else if area.height >= 14 {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        render_mounts_table(app, frame, chunks[0]);
        render_temp_files_table(app, frame, chunks[1]);
    } else {
        render_temp_files_table(app, frame, area);
    }
}




pub fn render_mounts_table(app: &App, frame: &mut Frame, area: Rect) {
    let theme = &app.theme;
    let is_wide = area.width >= 90;

    let table = if is_wide {
        let header_cells = ["Mount", "Model / Device", "Type", "FS", "Total", "Used", "Free", "Use %"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));
        let header = Row::new(header_cells).height(1);

        let rows = app.metrics.disk_list.iter().map(|d| {
            let usage_color = match d.usage_percent as u64 {
                0..=74 => theme.success,
                75..=89 => theme.warning,
                _ => theme.critical,
            };

            let kind_color = if d.disk_kind.contains("NVMe") || d.disk_kind.contains("M.2") {
                theme.primary
            } else if d.disk_kind.contains("SSD") {
                theme.success
            } else if d.disk_kind.contains("HDD") {
                theme.warning
            } else {
                theme.text_muted
            };

            Row::new(vec![
                Cell::from(d.mount_point.clone()).style(Style::default().add_modifier(Modifier::BOLD)),
                Cell::from(d.name.clone()),
                Cell::from(d.disk_kind.clone()).style(Style::default().fg(kind_color).add_modifier(Modifier::BOLD)),
                Cell::from(d.file_system.clone()).style(Style::default().fg(theme.text_muted)),
                Cell::from(format_bytes(d.total_space)),
                Cell::from(format_bytes(d.used_space)),
                Cell::from(format_bytes(d.free_space)),
                Cell::from(format!("{:.1}%", d.usage_percent)).style(Style::default().fg(usage_color)),
            ])
        });

        Table::new(
            rows,
            [
                Constraint::Length(7),
                Constraint::Min(16),
                Constraint::Length(10),
                Constraint::Length(8),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(8),
            ],
        )
        .header(header)
    } else {
        // Compact 2-line mode: prevent text cutoff on narrow columns
        let header_cells = ["Mount / Device", "Type / FS", "Used / Total", "Free / Use%"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));
        let header = Row::new(header_cells).height(1);

        let rows = app.metrics.disk_list.iter().map(|d| {
            let usage_color = match d.usage_percent as u64 {
                0..=74 => theme.success,
                75..=89 => theme.warning,
                _ => theme.critical,
            };

            let kind_color = if d.disk_kind.contains("NVMe") || d.disk_kind.contains("M.2") {
                theme.primary
            } else if d.disk_kind.contains("SSD") {
                theme.success
            } else if d.disk_kind.contains("HDD") {
                theme.warning
            } else {
                theme.text_muted
            };

            let cell_mount_dev = Cell::from(Text::from(vec![
                Line::from(vec![
                    Span::styled(d.mount_point.clone(), Style::default().add_modifier(Modifier::BOLD).fg(theme.primary)),
                    Span::raw(" "),
                    Span::styled(format!("({})", d.file_system), Style::default().fg(theme.text_muted)),
                ]),
                Line::from(Span::styled(d.name.clone(), Style::default().fg(theme.text_muted))),
            ]));

            let cell_type_fs = Cell::from(Text::from(vec![
                Line::from(Span::styled(d.disk_kind.clone(), Style::default().fg(kind_color).add_modifier(Modifier::BOLD))),
                Line::from(Span::styled(format!("FS: {}", d.file_system), Style::default().fg(theme.text_muted))),
            ]));

            let cell_space = Cell::from(Text::from(vec![
                Line::from(vec![
                    Span::styled("Usd: ", Style::default().fg(theme.text_muted)),
                    Span::raw(format_bytes(d.used_space)),
                ]),
                Line::from(vec![
                    Span::styled("Tot: ", Style::default().fg(theme.text_muted)),
                    Span::raw(format_bytes(d.total_space)),
                ]),
            ]));

            let cell_free_pct = Cell::from(Text::from(vec![
                Line::from(vec![
                    Span::styled("Lib: ", Style::default().fg(theme.text_muted)),
                    Span::raw(format_bytes(d.free_space)),
                ]),
                Line::from(Span::styled(format!("{:.1}%", d.usage_percent), Style::default().fg(usage_color).add_modifier(Modifier::BOLD))),
            ]));

            Row::new(vec![cell_mount_dev, cell_type_fs, cell_space, cell_free_pct]).height(2)
        });

        Table::new(
            rows,
            [
                Constraint::Min(16),
                Constraint::Length(12),
                Constraint::Length(14),
                Constraint::Length(12),
            ],
        )
        .header(header)
    };

    let table = table.block(
        Block::default()
            .title(Span::styled(
                " Storage Volumes & Mounts ",
                Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme.border_inactive)),
    );

    frame.render_widget(table, area);
}

pub fn render_smart_table(app: &App, frame: &mut Frame, area: Rect) {
    let theme = &app.theme;
    let is_wide = area.width >= 110;

    let table = if is_wide {
        let header_cells = [
            "Disk Drive",
            "Health Status",
            "Temp",
            "Power-On Time",
            "Power Cycles",
            "Host Reads",
            "Host Writes",
            "Serial / Firmware",
        ]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)));
        let header = Row::new(header_cells).height(1);

        let rows = app.metrics.smart_disks.iter().map(|s| {
            let (health_bullet, health_color) = if s.health_percent >= 90 {
                ("● ", theme.success)
            } else if s.health_percent >= 70 {
                ("▲ ", theme.warning)
            } else {
                ("✖ ", theme.critical)
            };

            let temp_str = s.temperature_c.map(|t| format!("{:.0} °C", t)).unwrap_or_else(|| "N/A".to_string());
            let days = s.power_on_hours / 24;
            let poh_str = format!("{} hrs ({}d)", s.power_on_hours, days);
            let poc_str = format!("{} veces", s.power_on_count);
            let reads_str = format_gb_tb(s.host_reads_gb);
            let writes_str = format_gb_tb(s.host_writes_gb);
            let sn_fw = format!("SN: {} │ FW: {}", s.serial_number, s.firmware);

            Row::new(vec![
                Cell::from(format!("{} ({})", s.model, s.media_type)).style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
                Cell::from(format!("{}{}", health_bullet, s.health_status)).style(Style::default().fg(health_color).add_modifier(Modifier::BOLD)),
                Cell::from(temp_str).style(Style::default().fg(theme.primary)),
                Cell::from(poh_str).style(Style::default().fg(theme.warning)),
                Cell::from(poc_str).style(Style::default().fg(theme.secondary)),
                Cell::from(reads_str).style(Style::default().fg(theme.success)),
                Cell::from(writes_str).style(Style::default().fg(theme.secondary)),
                Cell::from(sn_fw).style(Style::default().fg(theme.text_muted)),
            ])
        });

        Table::new(
            rows,
            [
                Constraint::Min(20),
                Constraint::Length(14),
                Constraint::Length(8),
                Constraint::Length(16),
                Constraint::Length(12),
                Constraint::Length(12),
                Constraint::Length(12),
                Constraint::Length(24),
            ],
        )
        .header(header)
    } else {
        // Compact 2-line mode for SMART metrics
        let header_cells = ["Disk Drive / Model", "Health & Temp", "Power-On / Cycles", "R/W & S/N"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)));
        let header = Row::new(header_cells).height(1);

        let rows = app.metrics.smart_disks.iter().map(|s| {
            let (health_bullet, health_color) = if s.health_percent >= 90 {
                ("● ", theme.success)
            } else if s.health_percent >= 70 {
                ("▲ ", theme.warning)
            } else {
                ("✖ ", theme.critical)
            };

            let temp_str = s.temperature_c.map(|t| format!("{:.0} °C", t)).unwrap_or_else(|| "N/A".to_string());
            let days = s.power_on_hours / 24;
            let poh_str = format!("{}h ({}d)", s.power_on_hours, days);
            let poc_str = format!("{} ciclos", s.power_on_count);
            let reads_str = format_gb_tb(s.host_reads_gb);
            let writes_str = format_gb_tb(s.host_writes_gb);
            let sn_fw = format!("SN: {} │ FW: {}", s.serial_number, s.firmware);

            let cell_drive = Cell::from(Text::from(vec![
                Line::from(Span::styled(s.model.clone(), Style::default().fg(theme.primary).add_modifier(Modifier::BOLD))),
                Line::from(Span::styled(s.media_type.clone(), Style::default().fg(theme.text_muted))),
            ]));

            let cell_health = Cell::from(Text::from(vec![
                Line::from(Span::styled(format!("{}{}", health_bullet, s.health_status), Style::default().fg(health_color).add_modifier(Modifier::BOLD))),
                Line::from(Span::styled(format!("Temp: {}", temp_str), Style::default().fg(theme.primary))),
            ]));

            let cell_power = Cell::from(Text::from(vec![
                Line::from(vec![
                    Span::styled("On: ", Style::default().fg(theme.text_muted)),
                    Span::styled(poh_str, Style::default().fg(theme.warning)),
                ]),
                Line::from(Span::styled(poc_str, Style::default().fg(theme.secondary))),
            ]));

            let cell_rw_sn = Cell::from(Text::from(vec![
                Line::from(vec![
                    Span::styled(format!("R:{} W:{}", reads_str, writes_str), Style::default().fg(theme.success)),
                ]),
                Line::from(Span::styled(sn_fw, Style::default().fg(theme.text_muted))),
            ]));

            Row::new(vec![cell_drive, cell_health, cell_power, cell_rw_sn]).height(2)
        });

        Table::new(
            rows,
            [
                Constraint::Min(16),
                Constraint::Length(14),
                Constraint::Length(14),
                Constraint::Length(22),
            ],
        )
        .header(header)
    };

    let table = table.block(
        Block::default()
            .title(Span::styled(
                " Disk Health & SMART ",
                Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme.border_inactive)),
    );

    frame.render_widget(table, area);
}

pub fn render_temp_files_table(app: &App, frame: &mut Frame, area: Rect) {
    let theme = &app.theme;
    let is_wide = area.width >= 95;

    let scan_indicator = if app.temp_files.is_scanning {
        Span::styled(" [Escaneando... ⟳] ", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" [t] Actualizar ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD))
    };

    let total_mb = app.temp_files.total_size_bytes as f64 / (1024.0 * 1024.0);
    let total_str = format_file_size(app.temp_files.total_size_bytes);

    let ago_str = app.temp_files.last_scan_time.map(|t| {
        let secs = t.elapsed().as_secs();
        if secs >= 60 {
            format!(" │ hace {}m", secs / 60)
        } else {
            format!(" │ hace {}s", secs)
        }
    }).unwrap_or_default();

    let summary_span = Span::styled(
        format!(" Total: {} │ {} archivos{} ", total_str, app.temp_files.total_file_count, ago_str),
        Style::default().fg(if total_mb >= 5000.0 { theme.critical } else if total_mb >= 1000.0 { theme.warning } else { theme.success }).add_modifier(Modifier::BOLD),
    );

    let title_line = Line::from(vec![
        Span::styled(" Temporary Files & System Cache ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        Span::raw("─"),
        summary_span,
        Span::raw("─"),
        scan_indicator,
    ]);

    let block = Block::default()
        .title(title_line)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_inactive));

    if is_wide {
        let header_cells = ["Location / Scope", "Path", "Files", "Size", "Status"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));
        let header = Row::new(header_cells).height(1);

        let mut rows: Vec<Row> = app.temp_files.locations.iter().map(|loc| {
            let status_color = if loc.is_accessible {
                theme.success
            } else if loc.status.contains("restringido") {
                theme.warning
            } else {
                theme.text_muted
            };

            let size_color = if loc.size_bytes >= 3 * 1024 * 1024 * 1024 {
                theme.critical
            } else if loc.size_bytes >= 1024 * 1024 * 1024 {
                theme.warning
            } else {
                theme.primary
            };

            Row::new(vec![
                Cell::from(loc.name.clone()).style(Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)),
                Cell::from(loc.path.clone()).style(Style::default().fg(theme.text_muted)),
                Cell::from(format!("{} items", loc.file_count)),
                Cell::from(format_file_size(loc.size_bytes)).style(Style::default().fg(size_color).add_modifier(Modifier::BOLD)),
                Cell::from(loc.status.clone()).style(Style::default().fg(status_color)),
            ])
        }).collect();

        // Summary row at the bottom
        if !app.temp_files.locations.is_empty() {
            rows.push(Row::new(vec![
                Cell::from("TOTAL ACUMULADO").style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
                Cell::from(format!("{} ubicaciones monitoreadas", app.temp_files.locations.len())).style(Style::default().fg(theme.text_muted)),
                Cell::from(format!("{} archivos", app.temp_files.total_file_count)).style(Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)),
                Cell::from(format_file_size(app.temp_files.total_size_bytes)).style(Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
                Cell::from("Listo").style(Style::default().fg(theme.success)),
            ]).style(Style::default().add_modifier(Modifier::UNDERLINED)));
        }

        let table = Table::new(
            rows,
            [
                Constraint::Length(24),
                Constraint::Min(25),
                Constraint::Length(14),
                Constraint::Length(12),
                Constraint::Length(14),
            ],
        )
        .header(header)
        .block(block);

        frame.render_widget(table, area);
    } else {
        // Compact mode for narrower terminals
        let header_cells = ["Location / Path", "Items / Size", "Status"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));
        let header = Row::new(header_cells).height(1);

        let rows = app.temp_files.locations.iter().map(|loc| {
            let status_color = if loc.is_accessible {
                theme.success
            } else if loc.status.contains("restringido") {
                theme.warning
            } else {
                theme.text_muted
            };

            let cell_loc = Cell::from(Text::from(vec![
                Line::from(Span::styled(loc.name.clone(), Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD))),
                Line::from(Span::styled(loc.path.clone(), Style::default().fg(theme.text_muted))),
            ]));

            let cell_stats = Cell::from(Text::from(vec![
                Line::from(Span::styled(format_file_size(loc.size_bytes), Style::default().fg(theme.warning).add_modifier(Modifier::BOLD))),
                Line::from(Span::raw(format!("{} items", loc.file_count))),
            ]));

            let cell_status = Cell::from(Span::styled(loc.status.clone(), Style::default().fg(status_color)));

            Row::new(vec![cell_loc, cell_stats, cell_status]).height(2)
        });

        let table = Table::new(
            rows,
            [
                Constraint::Min(20),
                Constraint::Length(16),
                Constraint::Length(14),
            ],
        )
        .header(header)
        .block(block);

        frame.render_widget(table, area);
    }
}

