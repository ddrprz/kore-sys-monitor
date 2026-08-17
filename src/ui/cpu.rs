use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Gauge, Sparkline},
    Frame,
};

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let theme = &app.theme;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(0)])
        .split(area);

    // Global CPU
    let global_val = app
        .metrics
        .global_cpu_history
        .back()
        .copied()
        .unwrap_or(0);

    let color = match global_val {
        0..=59 => theme.success,
        60..=84 => theme.warning,
        _ => theme.critical,
    };

    let sparkline_data: Vec<u64> = app.metrics.global_cpu_history.iter().copied().collect();

    let global_block = Block::default()
        .title(Span::styled(
            format!(" CPU Global [ {}% ] ", global_val),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_inactive));

    let sparkline = Sparkline::default()
        .block(global_block)
        .data(&sparkline_data)
        .max(100)
        .style(Style::default().fg(color));

    frame.render_widget(sparkline, chunks[0]);

    // Per-core Load
    let cores = &app.metrics.per_core_cpu;
    if !cores.is_empty() {
        let num_cores = cores.len();
        let cols = if num_cores > 16 {
            4
        } else if num_cores > 8 {
            2
        } else {
            1
        };

        let core_rows = (num_cores + cols - 1) / cols;

        let per_core_block = Block::default()
            .title(Span::styled(
                format!(" Per-Core Load ({} Cores) ", num_cores),
                Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme.border_inactive));

        let inner_area = per_core_block.inner(chunks[1]);
        frame.render_widget(per_core_block, chunks[1]);

        let col_constraints = vec![Constraint::Ratio(1, cols as u32); cols];
        let col_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints)
            .split(inner_area);

        for (idx, &usage) in cores.iter().enumerate() {
            let col_idx = idx % cols;
            let row_idx = idx / cols;

            let row_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(1); core_rows])
                .split(col_chunks[col_idx]);

            if row_idx < row_layout.len() {
                let core_color = match usage as u64 {
                    0..=59 => theme.success,
                    60..=84 => theme.warning,
                    _ => theme.critical,
                };

                let gauge = Gauge::default()
                    .gauge_style(Style::default().fg(core_color).bg(Color::Reset))
                    .ratio((usage as f64 / 100.0).clamp(0.0, 1.0))
                    .label(format!("Core {:2}: {:5.1}%", idx, usage));

                frame.render_widget(gauge, row_layout[row_idx]);
            }
        }
    }
}
