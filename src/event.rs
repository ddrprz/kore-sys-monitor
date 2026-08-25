use crate::app::{App, InputMode, Tab};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::io::Result;
use std::time::Duration;

pub fn handle_events(app: &mut App, tick_rate: Duration) -> Result<()> {

    if event::poll(tick_rate)?
        && let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                return Ok(());
            }
            match app.input_mode {
                InputMode::Normal => match (key.code, key.modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) | (KeyCode::Char('q'), _) => {
                        app.should_quit = true;
                    }
                    (KeyCode::Tab, _) => app.next_tab(),
                    (KeyCode::BackTab, _) => app.previous_tab(),
                    (KeyCode::Char('1'), _) => app.select_tab(Tab::Overview),
                    (KeyCode::Char('2'), _) => app.select_tab(Tab::Processes),
                    (KeyCode::Char('3'), _) => app.select_tab(Tab::StorageNet),
                    (KeyCode::Char('4'), _) => app.select_tab(Tab::CpuDetail),
                    (KeyCode::Char('5'), _) => app.select_tab(Tab::GpuDetail),
                    (KeyCode::Down, _) | (KeyCode::Char('j'), _) => app.select_next_process(),
                    (KeyCode::Up, _) | (KeyCode::Char('k'), _) => app.select_previous_process(),
                    (KeyCode::PageDown, _) => {
                        for _ in 0..10 {
                            app.select_next_process();
                        }
                    }
                    (KeyCode::PageUp, _) => {
                        for _ in 0..10 {
                            app.select_previous_process();
                        }
                    }
                    (KeyCode::Home, _) => app.selected_process_index = 0,
                    (KeyCode::End, _) => {
                        let count = app.filtered_sorted_processes().len();
                        if count > 0 {
                            app.selected_process_index = count - 1;
                        }
                    }
                    (KeyCode::Char('/'), _) => {
                        app.input_mode = InputMode::Searching;
                    }
                    (KeyCode::Char('s'), _) => app.cycle_sort(),
                    (KeyCode::Char('r'), _) => app.reverse_sort(),
                    (KeyCode::Char('t'), _) => app.cycle_theme(),
                    (KeyCode::Char('K'), _) | (KeyCode::Delete, _) => app.open_kill_modal(),

                    (KeyCode::Char('?'), _) => app.input_mode = InputMode::HelpModal,
                    (KeyCode::Esc, _) if !app.search_query.is_empty() => {
                        app.search_query.clear();
                    }
                    _ => {}
                },

                InputMode::Searching => match key.code {
                    KeyCode::Enter | KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Backspace => {
                        app.search_query.pop();
                    }
                    KeyCode::Char(c) => {
                        app.search_query.push(c);
                    }
                    _ => {}
                },

                InputMode::KillModal => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                        app.confirm_kill();
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        app.cancel_modal();
                    }
                    _ => {}
                },

                InputMode::HelpModal => match key.code {
                    KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    Ok(())
}
