use crate::app::{Bingo, cell::Cell, direction::Direction};

pub struct App {
    title: String,
    win_message: String,
    grid_size: usize,
    cells: Vec<Cell>,
    cursor_position: (usize, usize),
    free_cell: (usize, usize),
    game_won: bool,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            title: "Default bingo".to_string(),
            win_message: "You won! 🎉".to_string(),
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

    pub fn from(bingo: &Bingo) -> Self {
        let mut app = App {
            title: bingo.title().to_string(),
            win_message: bingo.win_message().to_string(),
            grid_size: bingo.grid_size(),
            cells: bingo.cells().clone(),
            free_cell: bingo.free_cell(),
            game_won: false,
            cursor_position: (0, 0),
        };

        app.ensure_free_cell_marked();
        app
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn win_message(&self) -> &str {
        &self.win_message
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

    pub fn move_cursor(&mut self, direction: &Direction) {
        let (move_x, move_y) = direction.to_vector();
        let new_x = (self.cursor_position.0.cast_signed() + move_x)
            .clamp(0, (self.grid_size - 1).cast_signed())
            .cast_unsigned();
        let new_y = (self.cursor_position.1.cast_signed() + move_y)
            .clamp(0, (self.grid_size - 1).cast_signed())
            .cast_unsigned();
        self.cursor_position = (new_x, new_y);
    }

    fn free_index(&self) -> usize {
        self.free_cell.1 * self.grid_size + self.free_cell.0
    }

    fn ensure_free_cell_marked(&mut self) {
        let free_index = self.free_index();
        if let Some(free_cell) = self.cells.get_mut(free_index)
            && !free_cell.marked()
        {
            free_cell.toggle();
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
            .any(|row| row.iter().all(Cell::marked))
    }

    fn won_by_column(&mut self) -> bool {
        (0..self.grid_size).any(|col_idx| {
            self.cells
                .iter()
                .skip(col_idx)
                .step_by(self.grid_size)
                .all(Cell::marked)
        })
    }

    fn won_by_diagonal(&mut self) -> bool {
        let diag1_win = (0..self.grid_size).all(|i| {
            let index = i * self.grid_size + i;
            self.cells.get(index).is_some_and(Cell::marked)
        });

        let diag2_win = (0..self.grid_size).all(|i| {
            let index = i * self.grid_size + (self.grid_size - 1 - i);
            self.cells.get(index).is_some_and(Cell::marked)
        });

        diag1_win || diag2_win
    }

    fn check_game_won(&mut self) {
        self.game_won = self.won_by_row() || self.won_by_column() || self.won_by_diagonal();
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
        self.game_won = false;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
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
        app.move_cursor(&Direction::Right);
        assert_eq!(app.cursor_position(), (1, 0));
        app.move_cursor(&Direction::Down);
        assert_eq!(app.cursor_position(), (1, 1));
        app.move_cursor(&Direction::Left);
        assert_eq!(app.cursor_position(), (0, 1));
        app.move_cursor(&Direction::Up);
        assert_eq!(app.cursor_position(), (0, 0));

        // Test boundary conditions
        app.move_cursor(&Direction::Up);
        assert_eq!(app.cursor_position(), (0, 0));
        app.move_cursor(&Direction::Left);
        assert_eq!(app.cursor_position(), (0, 0));
        for _ in 0..10 {
            app.move_cursor(&Direction::Down);
        }
        assert_eq!(app.cursor_position(), (0, 4));
        for _ in 0..10 {
            app.move_cursor(&Direction::Right);
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

        app.move_cursor(&Direction::Right);
        app.move_cursor(&Direction::Down);
        app.move_cursor(&Direction::Right);
        app.move_cursor(&Direction::Down);
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
        app.move_cursor(&Direction::Right);
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

    #[test]
    fn test_game_won_by_diagonal_1() {
        let mut app = App::new();
        for i in 0..app.grid_size() {
            app.cursor_position = (i, i);
            app.toggle_current_cell();
        }
        assert!(app.game_won());
    }

    #[test]
    fn test_game_won_by_diagonal_2() {
        let mut app = App::new();
        let grid_size = app.grid_size();
        for i in 0..grid_size {
            app.cursor_position = (grid_size - 1 - i, i);
            app.toggle_current_cell();
        }
        assert!(app.game_won());
    }

    #[test]
    fn test_reset_after_win() {
        let mut app = App::new();
        for x in 0..app.grid_size() {
            app.cursor_position = (x, 0);
            app.toggle_current_cell();
        }
        assert!(app.game_won());
        app.reset();
        assert!(!app.game_won());
    }
}
