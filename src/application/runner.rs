use std::time::{Duration, Instant};
use anyhow::Result;
use crossterm::event::{self, Event};

use crate::domain::App;
use crate::infrastructure::terminal::Tui;
use crate::presentation::tui::render;
use crate::application::event_handler::{handle_key_event, handle_mouse_event};

pub fn run(terminal: &mut Tui, mut app: App) -> Result<()> {
    // Timing Constants
    let animation_tick = Duration::from_millis(150);
    let mut last_animation = Instant::now();
    let mut last_second = Instant::now();

    // Efficient Event-Driven Loop
    while !app.should_quit {
        // Render only when needed
        terminal.draw(|f| render(f, &app))?;

        // Calculate the precise time until the next animation frame
        let next_update = animation_tick
            .checked_sub(last_animation.elapsed())
            .unwrap_or(Duration::from_millis(10)); // Minimum fallback

        // Wait for an event OR the timeout
        if event::poll(next_update)? {
            // Drain all events to keep input snappy
            while event::poll(Duration::from_secs(0))? {
                match event::read()? {
                    Event::Key(key) => handle_key_event(&mut app, key),
                    Event::Mouse(mouse) => handle_mouse_event(&mut app, mouse),
                    Event::Resize(_, _) => {}, // Re-render on resize
                    _ => {}
                }
            }
        }

        // Logic Updates (Only when time thresholds are crossed)
        if last_animation.elapsed() >= animation_tick {
            app.update_frame();
            last_animation = Instant::now();
        }

        if last_second.elapsed() >= Duration::from_secs(1) {
            app.tick();
            last_second = Instant::now();
        }
    }

    Ok(())
}
