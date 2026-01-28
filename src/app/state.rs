use crate::app::{cell::Cell, direction::Direction};

pub struct App {
    grid_size: usize,
    cells: Vec<Cell>,
    cursor_position: (usize, usize),
}

impl App {
    pub fn new() -> Self {
        App {
            grid_size: 5,
            cells: (0..25)
                .map(|i| Cell::new(&format!("Cell {}", i + 1)))
                .collect(),
            cursor_position: (0, 0),
        }
    }

    pub fn grid_size(&self) -> usize {
        self.grid_size
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        self.cursor_position
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        let (move_x, move_y) = direction.to_vector();
        let new_x = (self.cursor_position.0 as isize + move_x)
            .clamp(0, (self.grid_size - 1) as isize) as usize;
        let new_y = (self.cursor_position.1 as isize + move_y)
            .clamp(0, (self.grid_size - 1) as isize) as usize;
        self.cursor_position = (new_x, new_y);
    }

    fn current_index(&self) -> usize {
        self.cursor_position.1 * self.grid_size + self.cursor_position.0
    }

    pub fn toggle_current_cell(&mut self) {
        let index = self.current_index();
        if let Some(cell) = self.cells.get_mut(index) {
            cell.toggle();
        }
    }

    pub fn reset(&mut self) {
        for cell in &mut self.cells {
            if cell.marked() {
                cell.toggle();
            }
        }
        self.cursor_position = (0, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert_eq!(app.grid_size(), 5);
        assert_eq!(app.cells().len(), 25);
        assert_eq!(app.cursor_position(), (0, 0));
    }

    #[test]
    fn test_move_cursor() {
        let mut app = App::new();
        app.move_cursor(Direction::Right);
        assert_eq!(app.cursor_position(), (1, 0));
        app.move_cursor(Direction::Down);
        assert_eq!(app.cursor_position(), (1, 1));
        app.move_cursor(Direction::Left);
        assert_eq!(app.cursor_position(), (0, 1));
        app.move_cursor(Direction::Up);
        assert_eq!(app.cursor_position(), (0, 0));

        // Test boundary conditions
        app.move_cursor(Direction::Up);
        assert_eq!(app.cursor_position(), (0, 0));
        app.move_cursor(Direction::Left);
        assert_eq!(app.cursor_position(), (0, 0));
        for _ in 0..10 {
            app.move_cursor(Direction::Down);
        }
        assert_eq!(app.cursor_position(), (0, 4));
        for _ in 0..10 {
            app.move_cursor(Direction::Right);
        }
        assert_eq!(app.cursor_position(), (4, 4));
    }

    #[test]
    fn test_toggle_current_cell() {
        let mut app = App::new();
        app.toggle_current_cell();
        let index = 0;
        assert!(app.cells()[index].marked());
        app.toggle_current_cell();
        assert!(!app.cells()[index].marked());
    }

    #[test]
    fn test_reset() {
        let mut app = App::new();
        app.toggle_current_cell();
        app.move_cursor(Direction::Right);
        app.toggle_current_cell();
        app.reset();
        for cell in app.cells() {
            assert!(!cell.marked());
        }
        assert_eq!(app.cursor_position(), (0, 0));
    }
}
