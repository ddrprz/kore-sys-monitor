use crate::app::App;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
    Frame,
};

fn format_bytes(bytes: u64) -> String {
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{:.1} GB", gb)
}

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let header_cells = ["Mount", "Type", "Total", "Used", "Free", "Use %"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1);

    let rows = app.metrics.disk_list.iter().map(|d| {
        let usage_color = match d.usage_percent as u64 {
            0..=74 => Color::Green,
            75..=89 => Color::Yellow,
            _ => Color::Red,
        };

        Row::new(vec![
            Cell::from(d.mount_point.clone()).style(Style::default().fg(Color::White)),
            Cell::from(d.file_system.clone()).style(Style::default().fg(Color::DarkGray)),
            Cell::from(format_bytes(d.total_space)),
            Cell::from(format_bytes(d.used_space)),
            Cell::from(format_bytes(d.free_space)),
            Cell::from(format!("{:.1}%", d.usage_percent)).style(Style::default().fg(usage_color)),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 6),
            Constraint::Ratio(1, 6),
            Constraint::Ratio(1, 6),
            Constraint::Ratio(1, 6),
            Constraint::Ratio(1, 12),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(Span::styled(
                " Disks & Mounts ",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    frame.render_widget(table, area);
}
