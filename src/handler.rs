use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.quiet();
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quiet();
            }
        }
        KeyCode::Right => {
            app.move_right();
        }
        KeyCode::Left => {
            app.move_left();
        }
        KeyCode::Down => {
            app.move_down();
        }
        KeyCode::Up => {
            app.rotate();
        }
        _ => {}
    }
}
