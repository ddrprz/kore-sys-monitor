use crate::app::App;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
    Frame,
};

fn format_bytes(bytes: u64) -> String {
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{:.1} GB", gb)
}

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let theme = &app.theme;

    let header_cells = ["Mount", "Model / Device", "Type", "Health", "FS", "Total", "Used", "Free", "Use %"]
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

        let health_color = if d.health.to_lowercase().contains("healthy") || d.health.to_lowercase().contains("ok") {
            theme.success
        } else if d.health.to_lowercase().contains("warn") || d.health.to_lowercase().contains("degrad") {
            theme.warning
        } else {
            theme.critical
        };

        Row::new(vec![
            Cell::from(d.mount_point.clone()).style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from(d.name.clone()),
            Cell::from(d.disk_kind.clone()).style(Style::default().fg(kind_color).add_modifier(Modifier::BOLD)),
            Cell::from(d.health.clone()).style(Style::default().fg(health_color).add_modifier(Modifier::BOLD)),
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
            Constraint::Length(8),
            Constraint::Min(16),
            Constraint::Length(10),
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
                " Storage & Mounts ",
                Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme.border_inactive)),
    );

    frame.render_widget(table, area);
}
