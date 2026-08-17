use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn render_kill_modal(app: &App, frame: &mut Frame, area: Rect) {
    if let Some(proc) = &app.selected_kill_process {
        let popup_area = centered_rect(50, 30, area);

        let text = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                " ¿Seguro que deseas terminar este proceso? ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  PID:   "),
                Span::styled(proc.pid.to_string(), Style::default().fg(Color::Yellow)),
            ]),
            Line::from(vec![
                Span::raw("  Name:  "),
                Span::styled(&proc.name, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("  CPU:   "),
                Span::styled(format!("{:.1}%", proc.cpu_usage), Style::default().fg(Color::Green)),
                Span::raw("   │   MEM: "),
                Span::styled(format!("{:.1}%", proc.memory_percent), Style::default().fg(Color::Magenta)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" [ Y / Enter: Confirmar ] ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw("   "),
                Span::styled(" [ N / Esc: Cancelar ] ", Style::default().fg(Color::DarkGray)),
            ]),
        ];

        let block = Block::default()
            .title(Span::styled(" Terminar Proceso ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Red));

        let paragraph = Paragraph::new(text).alignment(Alignment::Center).block(block);

        frame.render_widget(Clear, popup_area);
        frame.render_widget(paragraph, popup_area);
    }
}

pub fn render_help_modal(frame: &mut Frame, area: Rect) {
    let popup_area = centered_rect(60, 60, area);

    let text = vec![
        Line::from(Span::styled(" Atajos de Teclado ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![Span::styled(" Tab / Shift+Tab ", Style::default().fg(Color::Yellow)), Span::raw(" Navegar entre pestañas")]),
        Line::from(vec![Span::styled(" 1, 2, 3, 4      ", Style::default().fg(Color::Yellow)), Span::raw(" Selección directa de pestaña")]),
        Line::from(vec![Span::styled(" j / k (Down/Up) ", Style::default().fg(Color::Yellow)), Span::raw(" Seleccionar proceso siguiente / anterior")]),
        Line::from(vec![Span::styled(" PgUp / PgDown   ", Style::default().fg(Color::Yellow)), Span::raw(" Desplazamiento rápido por lista")]),
        Line::from(vec![Span::styled(" Home / End      ", Style::default().fg(Color::Yellow)), Span::raw(" Ir al inicio / final de la lista")]),
        Line::from(vec![Span::styled(" /               ", Style::default().fg(Color::Yellow)), Span::raw(" Buscar / Filtrar procesos en tiempo real")]),
        Line::from(vec![Span::styled(" s               ", Style::default().fg(Color::Yellow)), Span::raw(" Cambiar columna de ordenación (CPU/MEM/PID/Name)")]),
        Line::from(vec![Span::styled(" r               ", Style::default().fg(Color::Yellow)), Span::raw(" Invertir ordenación (Asc / Desc)")]),
        Line::from(vec![Span::styled(" K / Delete      ", Style::default().fg(Color::Yellow)), Span::raw(" Ventana modal para terminar proceso seleccionado")]),
        Line::from(vec![Span::styled(" ?               ", Style::default().fg(Color::Yellow)), Span::raw(" Abrir / Cerrar esta ventana de ayuda")]),
        Line::from(vec![Span::styled(" q / Ctrl+C      ", Style::default().fg(Color::Yellow)), Span::raw(" Salir de la aplicación limpiando terminal")]),
        Line::from(""),
        Line::from(Span::styled(" Presiona Esc, ? o q para cerrar ", Style::default().fg(Color::DarkGray))),
    ];

    let block = Block::default()
        .title(Span::styled(" Ayuda - kore-sys-monitor ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(text).alignment(Alignment::Center).block(block);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(paragraph, popup_area);
}
