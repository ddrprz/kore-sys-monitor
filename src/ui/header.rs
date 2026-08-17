use crate::app::App;
use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(app: &App, frame: &mut Frame, area: ratatui::layout::Rect) {
    let uptime_hours = app.metrics.uptime_secs / 3600;
    let uptime_mins = (app.metrics.uptime_secs % 3600) / 60;

    let header_text = vec![Line::from(vec![
        Span::styled(
            " kore-sys-monitor v0.1.0 ",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::raw("Host: "),
        Span::styled(&app.metrics.host_name, Style::default().fg(Color::Green)),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::raw("OS: "),
        Span::styled(&app.metrics.os_name, Style::default().fg(Color::Yellow)),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::raw("Kernel: "),
        Span::styled(&app.metrics.kernel_version, Style::default().fg(Color::Cyan)),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::raw("Arch: "),
        Span::styled(&app.metrics.cpu_arch, Style::default().fg(Color::Blue)),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::raw("Uptime: "),
        Span::styled(
            format!("{}h {}m", uptime_hours, uptime_mins),
            Style::default().fg(Color::Magenta),
        ),

    ])];

    let paragraph = Paragraph::new(header_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)),
        );

    frame.render_widget(paragraph, area);
}
