use crossterm::event::{KeyCode, KeyEvent};

use crate::app::Direction;

pub enum Action {
    MoveCursor(Direction),
    Toggle,
    Reset,
    Quit,
    None,
}

impl Action {
    pub fn from_key(key: KeyEvent) -> Self {
        match key.code {
            KeyCode::Up => Action::MoveCursor(Direction::Up),
            KeyCode::Char('k') => Action::MoveCursor(Direction::Up),
            KeyCode::Down => Action::MoveCursor(Direction::Down),
            KeyCode::Char('j') => Action::MoveCursor(Direction::Down),
            KeyCode::Left => Action::MoveCursor(Direction::Left),
            KeyCode::Char('h') => Action::MoveCursor(Direction::Left),
            KeyCode::Right => Action::MoveCursor(Direction::Right),
            KeyCode::Char('l') => Action::MoveCursor(Direction::Right),
            KeyCode::Enter => Action::Toggle,
            KeyCode::Char(' ') => Action::Toggle,
            KeyCode::Char('r') => Action::Reset,
            KeyCode::Char('q') => Action::Quit,
            _ => Action::None,
        }
    }
}
