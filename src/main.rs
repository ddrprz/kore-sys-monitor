mod app;
mod event;
mod system;
mod ui;

use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::{self, Stdout},
    panic,
    time::{Duration, Instant},
};

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() {
    let _ = disable_raw_mode();
    let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
}

fn set_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        restore_terminal();
        original_hook(panic_info);
    }));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Register custom panic hook to restore terminal on panics
    set_panic_hook();

    // 2. Setup Terminal UI
    let mut terminal = setup_terminal()?;

    // 3. Initialize Application State
    let mut app = App::new();

    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    // 4. Main Event & Render Loop
    while !app.should_quit {
        // Draw TUI frame
        terminal.draw(|frame| ui::render(&app, frame))?;

        // Calculate elapsed time for system metrics refresh
        let elapsed = last_tick.elapsed().as_secs_f64();

        // Handle Keyboard Events
        event::handle_events(&mut app, tick_rate)?;

        // Periodically refresh system metrics (~1 second intervals)
        if last_tick.elapsed() >= Duration::from_secs(1) {
            app.update(elapsed);
            last_tick = Instant::now();
        }
    }

    // 5. Restore Terminal before exiting
    restore_terminal();

    Ok(())
}
