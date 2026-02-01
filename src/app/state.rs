use crate::app::{cell::Cell, direction::Direction};

pub struct App {
    grid_size: usize,
    cells: Vec<Cell>,
    cursor_position: (usize, usize),
    free_cell: (usize, usize),
    game_won: bool,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            grid_size: 5,
            cells: (0..25)
                .map(|i| Cell::new(&format!("Cell {}", i + 1)))
                .collect(),
            cursor_position: (0, 0),
            free_cell: (2, 2),
            game_won: false,
        };

        // Not sure if this is the best way to do this...
        app.ensure_free_cell_marked();
        app
    }

    pub fn grid_size(&self) -> usize {
        self.grid_size
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn game_won(&self) -> bool {
        self.game_won
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

    fn free_index(&self) -> usize {
        self.free_cell.1 * self.grid_size + self.free_cell.0
    }

    fn ensure_free_cell_marked(&mut self) {
        let free_index = self.free_index();
        if let Some(free_cell) = self.cells.get_mut(free_index) {
            if !free_cell.marked() {
                free_cell.toggle();
            }
        }
    }

    pub fn current_index(&self) -> usize {
        self.cursor_position.1 * self.grid_size + self.cursor_position.0
    }

    pub fn current_cell(&self) -> Option<&Cell> {
        let index = self.current_index();
        self.cells.get(index)
    }

    fn won_by_row(&mut self) -> bool {
        self.cells
            .chunks(self.grid_size)
            .any(|row| row.iter().all(|cell| cell.marked()))
    }

    fn won_by_column(&mut self) -> bool {
        (0..self.grid_size).any(|col_idx| {
            self.cells
                .iter()
                .skip(col_idx)
                .step_by(self.grid_size)
                .all(|cell| cell.marked())
        })
    }

    fn check_game_won(&mut self) {
        let rows_win = self.won_by_row();
        let cols_win = self.won_by_column();

        self.game_won = rows_win || cols_win;
    }

    pub fn toggle_current_cell(&mut self) {
        let index = self.current_index();
        if let Some(cell) = self.cells.get_mut(index) {
            cell.toggle();
        }

        self.ensure_free_cell_marked();
        self.check_game_won();
    }

    pub fn reset(&mut self) {
        for cell in &mut self.cells {
            if cell.marked() {
                cell.toggle();
            }
        }
        self.ensure_free_cell_marked();
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
    fn test_free_cell_always_marked() {
        let mut app = App::new();
        let free_index = app.free_index();

        assert!(app.cells()[free_index].marked());

        app.move_cursor(Direction::Right);
        app.move_cursor(Direction::Down);
        app.move_cursor(Direction::Right);
        app.move_cursor(Direction::Down);
        assert!(app.cursor_position() == app.free_cell);

        assert!(app.cells()[free_index].marked());
        app.toggle_current_cell();
        assert!(app.cells()[free_index].marked());
        app.toggle_current_cell();
        assert!(app.cells()[free_index].marked());
    }

    #[test]
    fn test_reset() {
        let mut app = App::new();
        app.toggle_current_cell();
        app.move_cursor(Direction::Right);
        app.toggle_current_cell();
        app.reset();
        let free_index = app.free_index();
        assert!(app.cells()[free_index].marked());
        for (i, cell) in app.cells().iter().enumerate() {
            if i != free_index {
                assert!(!cell.marked());
            }
        }
        assert_eq!(app.cursor_position(), (0, 0));
        assert!(!app.game_won());
    }

    #[test]
    fn test_game_won_by_row() {
        let mut app = App::new();
        for x in 0..app.grid_size() {
            app.cursor_position = (x, 0);
            app.toggle_current_cell();
        }
        assert!(app.game_won());
    }

    #[test]
    fn test_game_won_by_column() {
        let mut app = App::new();
        for y in 0..app.grid_size() {
            app.cursor_position = (0, y);
            app.toggle_current_cell();
        }
        assert!(app.game_won());
    }
}
