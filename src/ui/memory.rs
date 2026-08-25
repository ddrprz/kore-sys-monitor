use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
    Frame,
};

fn format_bytes(bytes: u64) -> String {
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{:.1} GB", gb)
}

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let theme = &app.theme;

    let ram_spec = if app.metrics.ram_details.speed_mhz != "N/A" {
        format!(
            " Memory & Swap [{}] ",
            app.metrics.ram_details.memory_type
        )
    } else {
        " Memory & Swap ".to_string()
    };

    let block = Block::default()
        .title(Span::styled(
            ram_spec,
            Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_inactive));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // RAM Gauge
            Constraint::Length(2), // Swap Gauge
            Constraint::Min(1),    // Hardware details line
        ])
        .margin(1)
        .split(inner);

    // RAM Gauge
    let ram_used = app.metrics.memory_used;
    let ram_total = app.metrics.memory_total;
    let ram_ratio = if ram_total > 0 {
        (ram_used as f64 / ram_total as f64).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let ram_pct = ram_ratio * 100.0;

    let ram_color = match ram_pct as u64 {
        0..=59 => theme.success,
        60..=84 => theme.warning,
        _ => theme.critical,
    };

    let ram_label = format!(
        "RAM:  {} / {} ({:.1}%)",
        format_bytes(ram_used),
        format_bytes(ram_total),
        ram_pct
    );

    let ram_gauge = Gauge::default()
        .gauge_style(Style::default().fg(ram_color).bg(Color::Reset))
        .ratio(ram_ratio)
        .label(ram_label);

    frame.render_widget(ram_gauge, chunks[0]);

    // Swap Gauge
    let swap_used = app.metrics.swap_used;
    let swap_total = app.metrics.swap_total;
    let swap_ratio = if swap_total > 0 {
        (swap_used as f64 / swap_total as f64).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let swap_pct = swap_ratio * 100.0;

    let swap_color = match swap_pct as u64 {
        0..=59 => theme.secondary,
        60..=84 => theme.warning,
        _ => theme.critical,
    };

    let swap_label = format!(
        "Swap: {} / {} ({:.1}%)",
        format_bytes(swap_used),
        format_bytes(swap_total),
        swap_pct
    );

    let swap_gauge = Gauge::default()
        .gauge_style(Style::default().fg(swap_color).bg(Color::Reset))
        .ratio(swap_ratio)
        .label(swap_label);

    frame.render_widget(swap_gauge, chunks[1]);

    // Hardware RAM spec detail line
    if chunks.len() > 2 && chunks[2].height > 0 {
        let text = if chunks[2].width < 45 {
            vec![
                Line::from(vec![
                    Span::styled("Spec: ", Style::default().fg(theme.text_muted)),
                    Span::styled(
                        &app.metrics.ram_details.memory_type,
                        Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(" │ ", Style::default().fg(theme.text_muted)),
                    Span::raw("Speed: "),
                    Span::styled(
                        &app.metrics.ram_details.speed_mhz,
                        Style::default().fg(theme.secondary),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Vendor: ", Style::default().fg(theme.text_muted)),
                    Span::styled(
                        &app.metrics.ram_details.manufacturer,
                        Style::default().fg(theme.warning),
                    ),
                ]),
            ]
        } else {
            vec![Line::from(vec![
                Span::styled("RAM Spec: ", Style::default().fg(theme.text_muted)),
                Span::styled(
                    &app.metrics.ram_details.memory_type,
                    Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" │ ", Style::default().fg(theme.text_muted)),
                Span::raw("Speed: "),
                Span::styled(
                    &app.metrics.ram_details.speed_mhz,
                    Style::default().fg(theme.secondary),
                ),
                Span::styled(" │ ", Style::default().fg(theme.text_muted)),
                Span::raw("Vendor: "),
                Span::styled(
                    &app.metrics.ram_details.manufacturer,
                    Style::default().fg(theme.warning),
                ),
            ])]
        };

        let details_paragraph = Paragraph::new(text).wrap(ratatui::widgets::Wrap { trim: true });
        frame.render_widget(details_paragraph, chunks[2]);
    }
}
