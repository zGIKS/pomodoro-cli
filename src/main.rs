use std::{
    io::stdout,
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod dog;
mod ui;

use crate::app::App;

fn main() -> Result<()> {
    // Basic Terminal Setup
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let animation_tick = Duration::from_millis(200);
    let mut last_animation = Instant::now();

    while !app.should_quit {
        // Draw UI
        terminal.draw(|f| ui::render(f, &app))?;

        // Wait for next animation frame
        let timeout = animation_tick
            .checked_sub(last_animation.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // Basic exit events
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => app.quit(),
                    _ => {}
                }
            }
        }

        // Animation update
        if last_animation.elapsed() >= animation_tick {
            app.update_animation();
            last_animation = Instant::now();
        }
    }

    // Terminal Restore
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
