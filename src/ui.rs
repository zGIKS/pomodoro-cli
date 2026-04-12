use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::app::App;
use crate::dog;

/// Main UI rendering with "Cava-style" smooth progress bar
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
    let dog_frame = if app.frame_count % 2 == 0 {
        dog::FRAME_1
    } else {
        dog::FRAME_2
    };
    f.render_widget(Paragraph::new(dog_frame).alignment(Alignment::Center).yellow(), chunks[1]);

    // 2. Render "Cava-style" Horizontal Bar
    let width = 50; // Total width of the bar
    let filled = (app.progress as f32 / 100.0 * width as f32) as usize;
    
    // Create the bar with a gradient effect
    let mut spans = Vec::new();
    for i in 0..width {
        if i < filled {
            // Gradient from Cyan to Magenta
            let r = (100.0 + (i as f32 / width as f32) * 155.0) as u8;
            let g = (200.0 - (i as f32 / width as f32) * 100.0) as u8;
            let b = 255;
            spans.push(Span::styled("█", Style::default().fg(Color::Rgb(r, g, b))));
        } else {
            // Dark background for the empty part
            spans.push(Span::styled("░", Style::default().fg(Color::Rgb(40, 40, 40))));
        }
    }

    let progress_label = Line::from(vec![
        Span::styled(format!(" {} / {} ", app.work_duration, app.break_duration), Style::default().bold().white()),
        Span::styled(format!(" {}% ", app.progress), Style::default().dim()),
    ]).alignment(Alignment::Center);

    let bar_line = Line::from(spans).alignment(Alignment::Center);

    let progress_widget = Paragraph::new(vec![progress_label, Line::from(""), bar_line]);
    
    f.render_widget(progress_widget, chunks[2]);
}
