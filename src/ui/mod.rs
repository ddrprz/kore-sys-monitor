pub mod cpu;
pub mod gpu;
pub mod header;
pub mod memory;
pub mod modals;
pub mod network;
pub mod processes;
pub mod storage;

use crate::app::{App, InputMode, Tab};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Frame,
};

pub fn render(app: &App, frame: &mut Frame) {
    let size = frame.area();
    let theme = &app.theme;

    let is_small = size.width < 80 || size.height < 24;
    let header_height = if size.width < 90 {
        5
    } else if size.width < 140 {
        4
    } else {
        3
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(header_height), // Header
            Constraint::Length(3), // Tab navigation bar
            Constraint::Min(10),   // Main View Content
            Constraint::Length(1), // Footer status bar
        ])
        .split(size);

    // 1. Render Header
    header::render(app, frame, chunks[0]);

    // 2. Render Tabs (Compact for small screens)
    let tab_titles = vec![
        if is_small { Tab::Overview.compact_title() } else { Tab::Overview.title() },
        if is_small { Tab::Processes.compact_title() } else { Tab::Processes.title() },
        if is_small { Tab::StorageNet.compact_title() } else { Tab::StorageNet.title() },
        if is_small { Tab::CpuDetail.compact_title() } else { Tab::CpuDetail.title() },
        if is_small { Tab::GpuDetail.compact_title() } else { Tab::GpuDetail.title() },
    ];

    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.border_inactive)),
        )
        .select(app.active_tab as usize)
        .style(Style::default().fg(theme.text_muted))
        .highlight_style(
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(tabs, chunks[1]);

    // 3. Render Main View according to Active Tab & Breakpoints
    match app.active_tab {
        Tab::Overview => render_overview(app, frame, chunks[2]),
        Tab::Processes => processes::render(app, frame, chunks[2]),
        Tab::StorageNet => render_storage_net(app, frame, chunks[2]),
        Tab::CpuDetail => cpu::render_detail(app, frame, chunks[2]),
        Tab::GpuDetail => gpu::render_detail(app, frame, chunks[2]),
    }


    // 4. Render Footer Navigation Bar / Status Message
    render_footer(app, frame, chunks[3]);

    // 5. Render Overlay Modals (if active)
    if app.input_mode == InputMode::KillModal {
        modals::render_kill_modal(app, frame, size);
    } else if app.input_mode == InputMode::HelpModal {
        modals::render_help_modal(app, frame, size);
    }
}

fn render_overview(app: &App, frame: &mut Frame, area: Rect) {
    let is_ultrawide = area.width > 120 && area.height > 35;

    if is_ultrawide {
        // Ultra-wide 3-column layout: CPU (Left) | Memory, GPU & Net (Center) | Disks & Processes (Right)
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(35),
                Constraint::Percentage(35),
            ])
            .split(area);

        cpu::render(app, frame, cols[0]);

        let center_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(8), Constraint::Length(6), Constraint::Min(8)])
            .split(cols[1]);

        memory::render(app, frame, center_chunks[0]);
        gpu::render_summary(app, frame, center_chunks[1]);
        network::render(app, frame, center_chunks[2]);

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(cols[2]);

        storage::render(app, frame, right_chunks[0]);
        processes::render(app, frame, right_chunks[1]);
    } else {
        // Standard layout with CPU, Memory, GPU top row, Storage & Disk Health middle row, Processes bottom
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(9), // CPU, RAM, GPU
                Constraint::Length(8), // Storage Volumes & Disk Health
                Constraint::Min(8),    // Top Processes
            ])
            .split(area);

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(34),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(main_chunks[0]);

        cpu::render(app, frame, top_chunks[0]);
        memory::render(app, frame, top_chunks[1]);
        gpu::render_summary(app, frame, top_chunks[2]);
        storage::render_overview_disks(app, frame, main_chunks[1]);
        processes::render(app, frame, main_chunks[2]);
    }
}

fn render_storage_net(app: &App, frame: &mut Frame, area: Rect) {
    if area.height >= 26 {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
            .split(area);

        storage::render(app, frame, chunks[0]);
        network::render(app, frame, chunks[1]);
    } else {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        storage::render(app, frame, chunks[0]);
        network::render(app, frame, chunks[1]);
    }
}

fn render_footer(app: &App, frame: &mut Frame, area: Rect) {
    let theme = &app.theme;

    let footer_text = if let Some((msg, _)) = &app.status_message {
        Line::from(vec![
            Span::styled(" STATUS: ", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::styled(msg, Style::default().fg(theme.primary)),
        ])
    } else if area.width < 80 {
        Line::from(vec![
            Span::styled("[Tab]", Style::default().fg(theme.primary)),
            Span::raw("Nav │ "),
            Span::styled("[/]", Style::default().fg(theme.primary)),
            Span::raw("Search │ "),
            Span::styled("[t]", Style::default().fg(theme.primary)),
            Span::raw("Theme │ "),
            Span::styled("[Del]", Style::default().fg(theme.primary)),
            Span::raw("Kill │ "),
            Span::styled("[q]", Style::default().fg(theme.primary)),
            Span::raw("Quit"),
        ])
    } else {
        Line::from(vec![
            Span::styled(" [Tab]", Style::default().fg(theme.primary)),
            Span::raw(" Vista  │ "),
            Span::styled("[/]", Style::default().fg(theme.primary)),
            Span::raw(" Buscar  │ "),
            Span::styled("[s]", Style::default().fg(theme.primary)),
            Span::raw(" Ordenar  │ "),
            Span::styled("[r]", Style::default().fg(theme.primary)),
            Span::raw(" Invertir  │ "),
            Span::styled("[t]", Style::default().fg(theme.primary)),
            Span::raw(" Tema  │ "),
            Span::styled("[Del]", Style::default().fg(theme.primary)),
            Span::raw(" Matar Proceso  │ "),
            Span::styled("[?]", Style::default().fg(theme.primary)),
            Span::raw(" Ayuda  │ "),
            Span::styled("[q]", Style::default().fg(theme.primary)),
            Span::raw(" Salir"),
        ])
    };

    let paragraph = Paragraph::new(footer_text);
    frame.render_widget(paragraph, area);
}
