pub struct Cell {
    label: String,
    marked: bool,
}

impl Cell {
    pub fn new(label: &str) -> Self {
        Cell {
            label: label.to_string(),
            marked: false,
        }
    }
    fn label(&self) -> &str {
        &self.label
    }

    pub fn toggle(&mut self) {
        self.marked = !self.marked;
    }

    pub fn marked(&self) -> bool {
        self.marked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_creation() {
        let cell = Cell::new("Test");
        assert_eq!(cell.label(), "Test");
        assert!(!cell.marked());
    }

    #[test]
    fn test_toggle_mark() {
        let mut cell = Cell::new("Test");
        assert!(!cell.marked());
        cell.toggle();
        assert!(cell.marked());
        cell.toggle();
        assert!(!cell.marked());
    }
}
