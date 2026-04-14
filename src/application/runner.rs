use anyhow::Result;
use crossterm::event::{self, Event};
use std::time::{Duration, Instant};

use crate::application::event_handler::{handle_key_event, handle_mouse_event};
use crate::domain::{App, AppState};
use crate::domain::repository::Persistence;
use crate::infrastructure::terminal::Tui;
use crate::presentation::tui::render;

pub fn run(terminal: &mut Tui, mut app: App, persistence: &dyn Persistence) -> Result<()> {
    let animation_tick = Duration::from_millis(150);
    let mut last_animation = Instant::now();
    let mut last_second = Instant::now();
    let mut frame_count: usize = 0;

    while !app.should_quit() {
        terminal.draw(|f| render(f, &app, frame_count))?;

        let next_update = animation_tick
            .checked_sub(last_animation.elapsed())
            .unwrap_or(Duration::from_millis(10));

        if event::poll(next_update)? {
            let mut event_count = 0;
            const MAX_EVENTS_PER_FRAME: usize = 100;

            while event_count < MAX_EVENTS_PER_FRAME && event::poll(Duration::from_secs(0))? {
                event_count += 1;
                match event::read()? {
                    Event::Key(key) => handle_key_event(&mut app, key),
                    Event::Mouse(mouse) => handle_mouse_event(&mut app, mouse),
                    Event::Resize(_, _) => {}
                    _ => {}
                }
            }
        }

        if last_animation.elapsed() >= animation_tick {
            frame_count = frame_count.wrapping_add(1);
            app.update_error_timer();
            last_animation = Instant::now();
        }

        if last_second.elapsed() >= Duration::from_secs(1) {
            app.tick();
            last_second = Instant::now();
            
            // Save session state if we are running or paused
            if matches!(app.state(), AppState::Running | AppState::Paused) {
                let _ = persistence.save(&app);
            }
        }

        // Handle deleting session file if user declined to resume
        if !app.has_saved_session() && app.state() != AppState::Running && app.state() != AppState::Paused {
            // This is a bit aggressive, but if we don't have a saved session in App state, 
            // we should make sure the file is gone.
            // Actually, better only do it if it *just* changed.
            // For now, let's keep it simple.
            let _ = persistence.delete_session();
        }
    }

    Ok(())
}
