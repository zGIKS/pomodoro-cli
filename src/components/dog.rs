use ratatui::{
    layout::Alignment,
    style::Stylize,
    widgets::Paragraph,
    Frame,
};

use crate::app::App;

// Animated dog ASCII art frames
pub const FRAME_1: &str = r#"     .-.         
    {}``;        
    / ('         
(  /  |          
 \(_)_]]         "#;

pub const FRAME_2: &str = r#"     .-.         
    []``;        
    / ('         
 ) /  |          
 \(_)_]]         "#;

pub fn render(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let dog_frame = if app.frame_count % 2 == 0 {
        FRAME_1
    } else {
        FRAME_2
    };
    f.render_widget(Paragraph::new(dog_frame).alignment(Alignment::Center).yellow(), area);
}
