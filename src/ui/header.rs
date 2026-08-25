use crate::app::App;
use ratatui::{
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(app: &App, frame: &mut Frame, area: ratatui::layout::Rect) {
    let uptime_hours = app.metrics.uptime_secs / 3600;
    let uptime_mins = (app.metrics.uptime_secs % 3600) / 60;
    let theme = &app.theme;

    let mobo_str = format!("{} {}", app.metrics.motherboard.vendor, app.metrics.motherboard.model);

    let header_text = if area.width < 90 {
        vec![
            Line::from(vec![
                Span::styled("kore-sys v0.2.0 ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
                Span::styled("│ ", Style::default().fg(theme.text_muted)),
                Span::raw("Host: "),
                Span::styled(&app.metrics.host_name, Style::default().fg(theme.success)),
            ]),
            Line::from(vec![
                Span::raw("MB: "),
                Span::styled(&mobo_str, Style::default().fg(theme.warning)),
            ]),
            Line::from(vec![
                Span::raw("OS: "),
                Span::styled(&app.metrics.os_name, Style::default().fg(theme.warning)),
                Span::styled(" │ ", Style::default().fg(theme.text_muted)),
                Span::raw("Up: "),
                Span::styled(format!("{}h {}m", uptime_hours, uptime_mins), Style::default().fg(theme.secondary)),
                Span::styled(" │ ", Style::default().fg(theme.text_muted)),
                Span::styled(theme.variant.name(), Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
            ]),
        ]
    } else if area.width < 140 {
        vec![
            Line::from(vec![
                Span::styled("kore-sys v0.2.0 ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
                Span::styled("│ ", Style::default().fg(theme.text_muted)),
                Span::raw("Host: "),
                Span::styled(&app.metrics.host_name, Style::default().fg(theme.success)),
                Span::styled(" │ ", Style::default().fg(theme.text_muted)),
                Span::raw("MB: "),
                Span::styled(&mobo_str, Style::default().fg(theme.warning)),
            ]),
            Line::from(vec![
                Span::raw("OS: "),
                Span::styled(&app.metrics.os_name, Style::default().fg(theme.warning)),
                Span::styled(" │ ", Style::default().fg(theme.text_muted)),
                Span::raw("Up: "),
                Span::styled(format!("{}h {}m", uptime_hours, uptime_mins), Style::default().fg(theme.secondary)),
                Span::styled(" │ ", Style::default().fg(theme.text_muted)),
                Span::raw("Theme: "),
                Span::styled(theme.variant.name(), Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
            ]),
        ]
    } else {
        vec![Line::from(vec![
            Span::styled(
                " kore-sys-monitor v0.2.0 ",
                Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" │ ", Style::default().fg(theme.text_muted)),
            Span::raw("Host: "),
            Span::styled(&app.metrics.host_name, Style::default().fg(theme.success)),
            Span::styled(" │ ", Style::default().fg(theme.text_muted)),
            Span::raw("OS: "),
            Span::styled(&app.metrics.os_name, Style::default().fg(theme.warning)),
            Span::styled(" │ ", Style::default().fg(theme.text_muted)),
            Span::raw("Mobo: "),
            Span::styled(
                &mobo_str,
                Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" │ ", Style::default().fg(theme.text_muted)),
            Span::raw("Kernel: "),
            Span::styled(&app.metrics.kernel_version, Style::default().fg(theme.primary)),
            Span::styled(" │ ", Style::default().fg(theme.text_muted)),
            Span::raw("Arch: "),
            Span::styled(&app.metrics.cpu_arch, Style::default().fg(theme.secondary)),
            Span::styled(" │ ", Style::default().fg(theme.text_muted)),
            Span::raw("Uptime: "),
            Span::styled(
                format!("{}h {}m", uptime_hours, uptime_mins),
                Style::default().fg(theme.secondary),
            ),
            Span::styled(" │ ", Style::default().fg(theme.text_muted)),
            Span::raw("Theme: "),
            Span::styled(theme.variant.name(), Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        ])]
    };

    let paragraph = Paragraph::new(header_text)
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.border_active)),
        );

    frame.render_widget(paragraph, area);
}
