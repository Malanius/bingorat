use crate::app::cell::Cell;

pub struct Bingo {
    title: String,
    win_message: String,
    grid_size: usize,
    cells: Vec<Cell>,
    free_cell: (usize, usize),
}

impl Bingo {
    pub fn new(
        title: &str,
        win_message: &str,
        grid_size: usize,
        cells: Vec<Cell>,
        free_cell: (usize, usize),
    ) -> Self {
        Bingo {
            title: title.to_string(),
            win_message: win_message.to_string(),
            grid_size,
            cells,
            free_cell,
        }
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

    pub fn free_cell(&self) -> (usize, usize) {
        self.free_cell
    }
}
