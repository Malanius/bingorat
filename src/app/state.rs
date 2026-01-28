use crate::app::cell::Cell;

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
        }
    }
}
