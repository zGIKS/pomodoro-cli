use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::App;
use crate::components::{dog, progress_bar};

/// Main UI rendering
pub fn render(f: &mut Frame, app: &App) {
    let size = f.area();

    // Clean layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(8), // Dog
            Constraint::Length(4), // Cava-style Bar
            Constraint::Fill(1),
        ])
        .split(size);

    // 1. Render Dog
    dog::render(f, app, chunks[1]);

    // 2. Render Progress Bar
    progress_bar::render(f, app, chunks[2]);
}
