pub mod cpu;
pub mod header;
pub mod memory;
pub mod modals;
pub mod network;
pub mod processes;
pub mod storage;

use crate::app::{App, InputMode, Tab};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Frame,
};

pub fn render(app: &App, frame: &mut Frame) {
    let size = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Tab navigation bar
            Constraint::Min(10),   // Main View Content
            Constraint::Length(1), // Footer status bar
        ])
        .split(size);

    // 1. Render Header
    header::render(app, frame, chunks[0]);

    // 2. Render Tabs
    let tab_titles = vec![
        Tab::Overview.title(),
        Tab::Processes.title(),
        Tab::StorageNet.title(),
        Tab::CpuDetail.title(),
    ];

    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .select(app.active_tab as usize)
        .style(Style::default().fg(Color::DarkGray))
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(tabs, chunks[1]);

    // 3. Render Main View according to Active Tab
    match app.active_tab {
        Tab::Overview => render_overview(app, frame, chunks[2]),
        Tab::Processes => processes::render(app, frame, chunks[2]),
        Tab::StorageNet => render_storage_net(app, frame, chunks[2]),
        Tab::CpuDetail => cpu::render(app, frame, chunks[2]),
    }

    // 4. Render Footer Navigation Bar / Status Message
    render_footer(app, frame, chunks[3]);

    // 5. Render Overlay Modals (if active)
    if app.input_mode == InputMode::KillModal {
        modals::render_kill_modal(app, frame, size);
    } else if app.input_mode == InputMode::HelpModal {
        modals::render_help_modal(frame, size);
    }
}

fn render_overview(app: &App, frame: &mut Frame, area: Rect) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(8)])
        .split(area);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[0]);

    // Top Left: CPU
    cpu::render(app, frame, top_chunks[0]);

    // Top Right: Memory
    memory::render(app, frame, top_chunks[1]);

    // Bottom: Top Processes Table
    processes::render(app, frame, main_chunks[1]);
}

fn render_storage_net(app: &App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    storage::render(app, frame, chunks[0]);
    network::render(app, frame, chunks[1]);
}

fn render_footer(app: &App, frame: &mut Frame, area: Rect) {
    let footer_text = if let Some((msg, _)) = &app.status_message {
        Line::from(vec![
            Span::styled(" STATUS: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(msg, Style::default().fg(Color::White)),
        ])
    } else {
        Line::from(vec![
            Span::styled(" [Tab]", Style::default().fg(Color::Cyan)),
            Span::raw(" Vista  │ "),
            Span::styled("[/]", Style::default().fg(Color::Cyan)),
            Span::raw(" Buscar  │ "),
            Span::styled("[s]", Style::default().fg(Color::Cyan)),
            Span::raw(" Ordenar  │ "),
            Span::styled("[r]", Style::default().fg(Color::Cyan)),
            Span::raw(" Invertir  │ "),
            Span::styled("[K]", Style::default().fg(Color::Cyan)),
            Span::raw(" Matar Proceso  │ "),
            Span::styled("[?]", Style::default().fg(Color::Cyan)),
            Span::raw(" Ayuda  │ "),
            Span::styled("[q]", Style::default().fg(Color::Cyan)),
            Span::raw(" Salir"),
        ])
    };

    let paragraph = Paragraph::new(footer_text);
    frame.render_widget(paragraph, area);
}
