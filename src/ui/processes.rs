use crate::app::{App, InputMode, SortOrder};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};

fn format_mem(bytes: u64) -> String {
    let mb = bytes as f64 / (1024.0 * 1024.0);
    if mb >= 1024.0 {
        format!("{:.1} GB", mb / 1024.0)
    } else {
        format!("{:.1} MB", mb)
    }
}

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let procs = app.filtered_sorted_processes();
    let theme = &app.theme;

    let is_compact = area.width < 80;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(area);

    // Search bar header if searching or filter active
    let filter_text = if app.input_mode == InputMode::Searching {
        format!(" [SEARCH] Query: {} █ (Press Enter to apply)", app.search_query)
    } else if !app.search_query.is_empty() {
        format!(" [FILTER ACTIVE] Query: '{}' (Press Esc to clear)", app.search_query)
    } else {
        " Press / to filter processes".to_string()
    };

    let filter_paragraph = Paragraph::new(Line::from(vec![
        Span::styled(
            filter_text,
            if app.input_mode == InputMode::Searching {
                Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)
            } else if !app.search_query.is_empty() {
                Style::default().fg(theme.success)
            } else {
                Style::default().fg(theme.text_muted)
            },
        ),
    ]));
    frame.render_widget(filter_paragraph, chunks[0]);

    let order_symbol = match app.sort_order {
        SortOrder::Ascending => "▲",
        SortOrder::Descending => "▼",
    };

    let title_str = format!(
        " Processes ({}) [Sorted by: {} {}] ",
        procs.len(),
        app.sort_column.name(),
        order_symbol
    );

    let (header_cells, column_constraints): (Vec<&str>, Vec<Constraint>) = if is_compact {
        (
            vec!["PID", "NAME", "CPU%", "MEM%"],
            vec![
                Constraint::Length(7),
                Constraint::Length(18),
                Constraint::Length(7),
                Constraint::Length(7),
            ],
        )
    } else {
        (
            vec!["PID", "USER", "NAME", "CPU%", "MEM%", "MEM SIZE", "STATE", "COMMAND"],
            vec![
                Constraint::Length(8),
                Constraint::Length(8),
                Constraint::Length(20),
                Constraint::Length(8),
                Constraint::Length(8),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Min(20),
            ],
        )
    };

    let header_rows = header_cells
        .into_iter()
        .map(|h| Cell::from(h).style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_rows).height(1);

    let rows = procs.iter().enumerate().map(|(idx, p)| {
        let is_selected = idx == app.selected_process_index;

        let style = if is_selected {
            Style::default()
                .bg(theme.primary)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let cpu_color = if is_selected {
            Color::Black
        } else {
            match p.cpu_usage as u64 {
                0..=19 => theme.success,
                20..=59 => theme.warning,
                _ => theme.critical,
            }
        };

        if is_compact {
            Row::new(vec![
                Cell::from(p.pid.to_string()),
                Cell::from(p.name.clone()),
                Cell::from(format!("{:.1}%", p.cpu_usage)).style(Style::default().fg(cpu_color)),
                Cell::from(format!("{:.1}%", p.memory_percent)),
            ])
            .style(style)
        } else {
            Row::new(vec![
                Cell::from(p.pid.to_string()),
                Cell::from("user"),
                Cell::from(p.name.clone()),
                Cell::from(format!("{:.1}%", p.cpu_usage)).style(Style::default().fg(cpu_color)),
                Cell::from(format!("{:.1}%", p.memory_percent)),
                Cell::from(format_mem(p.memory)),
                Cell::from(p.status.clone()),
                Cell::from(p.command.clone()),
            ])
            .style(style)
        }
    });

    let table = Table::new(rows, column_constraints)
        .header(header)
        .block(
            Block::default()
                .title(Span::styled(
                    title_str,
                    Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.input_mode == InputMode::Searching {
                    theme.warning
                } else {
                    theme.border_inactive
                })),
        );

    let mut state = TableState::default();
    if !procs.is_empty() {
        state.select(Some(app.selected_process_index));
    }

    frame.render_stateful_widget(table, chunks[1], &mut state);
}
