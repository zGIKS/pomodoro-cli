use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
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
    
    f.render_widget(progress_widget, area);
}
