use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::domain::App;

pub fn render(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let menu_box_height = if app.has_saved_session() { 6 } else { 5 };
    let total_box_height = 4 + menu_box_height + 3;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(total_box_height),
            Constraint::Fill(1),
        ])
        .split(area);

    let inner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(44),
            Constraint::Fill(1),
        ])
        .split(chunks[1]);

    let header = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(" ⬢ ", Style::default().fg(Color::Cyan)),
            Span::styled("POMODOG", Style::default().bold().white()),
            Span::styled(" ⬢ ", Style::default().fg(Color::Cyan)),
        ]),
        Line::from("────────────────────────────────────────".dim()),
    ])
    .alignment(Alignment::Center);

    let mut options = Vec::new();
    
    // Resume option if available
    if app.has_saved_session() {
        if app.selected_index() == 0 {
            options.push(Line::from(vec![
                Span::styled("» ", Style::default().fg(Color::Yellow).bold()),
                Span::styled("RESUME PREVIOUS", Style::default().fg(Color::Green).bold()),
                Span::styled(" «", Style::default().fg(Color::Yellow).bold()),
            ]));
        } else {
            options.push(Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled("RESUME PREVIOUS", Style::default().fg(Color::Green).dim()),
                Span::styled("  ", Style::default()),
            ]));
        }
    }

    for (i, config) in app.configs().iter().enumerate() {
        let actual_idx = if app.has_saved_session() { i + 1 } else { i };
        
        if actual_idx == app.selected_index() {
            options.push(Line::from(vec![
                Span::styled("» ", Style::default().fg(Color::Yellow).bold()),
                Span::styled(&config.label, Style::default().fg(Color::White).bold()),
                Span::styled(" «", Style::default().fg(Color::Yellow).bold()),
            ]));
        } else {
            options.push(Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(&config.label, Style::default().fg(Color::DarkGray)),
                Span::styled("  ", Style::default()),
            ]));
        }
    }

    let menu_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let menu_widget = Paragraph::new(options)
        .block(menu_block)
        .alignment(Alignment::Center); // Keep content centered

    let footer = Paragraph::new(vec![
        Line::from(""),
        Line::from("▲▼ Move • Enter Start".dim()),
    ])
    .alignment(Alignment::Center);

    let list_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(menu_box_height),
            Constraint::Length(3),
        ])
        .split(inner_chunks[1]);

    f.render_widget(header, list_layout[0]);
    f.render_widget(menu_widget, list_layout[1]);
    f.render_widget(footer, list_layout[2]);
}
