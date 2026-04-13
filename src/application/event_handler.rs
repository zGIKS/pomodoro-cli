use crossterm::event::{self, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};
use crate::domain::{App, AppState};

pub fn handle_key_event(app: &mut App, key: event::KeyEvent) {
    if key.kind != KeyEventKind::Press { return; }

    match (key.code, key.modifiers, &app.state) {
        (KeyCode::Char('q'), _, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL, _) => app.quit(),

        (KeyCode::Up, _, AppState::Menu) => app.prev_option(),
        (KeyCode::Down, _, AppState::Menu) => app.next_option(),
        (KeyCode::Enter, _, AppState::Menu) => app.enter_task_input(),

        (KeyCode::Char(c), mods, AppState::TaskInput) => {
            if mods.is_empty() || mods == KeyModifiers::SHIFT {
                app.task_name.add_char(c);
            } else if mods == KeyModifiers::CONTROL && c == 'h' {
                // Many terminals send Ctrl+H for backspace
                app.task_name.remove_char();
            }
        }
        (KeyCode::Backspace, mods, AppState::TaskInput) => {
            if mods == KeyModifiers::CONTROL {
                app.task_name.remove_word();
            } else {
                app.task_name.remove_char();
            }
        }
        (KeyCode::Enter, _, AppState::TaskInput) => app.start_session(),
        (KeyCode::Esc, _, AppState::TaskInput) => app.state = AppState::Menu,

        (KeyCode::Char(' '), _, AppState::Running) | (KeyCode::Char(' '), _, AppState::Paused) => app.toggle_pause(),

        _ => {}
    }
}

pub fn handle_mouse_event(app: &mut App, mouse: event::MouseEvent) {
    if app.state == AppState::Menu {
        match mouse.kind {
            MouseEventKind::ScrollDown => app.next_option(),
            MouseEventKind::ScrollUp => app.prev_option(),
            _ => {}
        }
    }
}
