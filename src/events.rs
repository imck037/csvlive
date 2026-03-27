use crossterm::event::{KeyCode, KeyEvent};

use crate::{App, Mode};

pub fn handle_events(key: KeyEvent, app: &mut App) {
    match app.mode {
        Mode::Normal => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                app.move_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.move_down();
            }
            KeyCode::Left | KeyCode::Char('h') => {
                app.move_left();
            }
            KeyCode::Right | KeyCode::Char('l') => {
                app.move_right();
            }
            KeyCode::Char('i') => {
                app.start_edit();
            }
            _ => {}
        },
        Mode::Insert => match key.code {
            KeyCode::Enter => {
                app.save_edit();
            }
            KeyCode::Esc => {
                app.cancel_edit();
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            KeyCode::Char(c) => {
                app.input.push(c);
            }
            _ => {}
        },
    }
}
