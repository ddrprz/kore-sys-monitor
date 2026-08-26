use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
    Frame,
};

fn format_bytes(bytes: u64) -> String {
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{:.1} GB", gb)
}

fn format_gb_tb(gb: f64) -> String {
    if gb >= 1000.0 {
        format!("{:.1} TB", gb / 1024.0)
    } else {
        format!("{:.0} GB", gb)
    }
}

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    if area.height >= 14 {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(48),
                Constraint::Percentage(52),
            ])
            .split(area);

        render_mounts_table(app, frame, chunks[0]);
        render_smart_table(app, frame, chunks[1]);
    } else if area.width >= 110 {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        render_mounts_table(app, frame, chunks[0]);
        render_smart_table(app, frame, chunks[1]);
    } else {
        render_mounts_table(app, frame, area);
    }
}

pub fn render_overview_disks(app: &App, frame: &mut Frame, area: Rect) {
    if area.width >= 100 {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        render_mounts_table(app, frame, chunks[0]);
        render_smart_table(app, frame, chunks[1]);
    } else {
        render_mounts_table(app, frame, area);
    }
}

pub fn render_mounts_table(app: &App, frame: &mut Frame, area: Rect) {
    let theme = &app.theme;

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

    let table = Table::new(
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
    .block(
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

    let table = Table::new(
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
    .block(
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
