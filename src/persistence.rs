use std::{fs, path::PathBuf};

use crate::app::{Bingo, cell::Cell};

pub fn parse_predictions(path: &str) -> Bingo {
    let file = PathBuf::from(path);
    let contents = fs::read_to_string(file).expect("Failed to read file");

    let filtered_contents = contents
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .collect::<Vec<&str>>();

    let mut title = String::new();
    let mut win_message = String::new();
    let mut size = 0;
    let mut cells: Vec<Cell> = Vec::new();

    filtered_contents.into_iter().for_each(|line| match line {
        s if s.starts_with("size:") => size = parse_size(s),
        t if t.starts_with("title:") => title = parse_string_param(t),
        w if w.starts_with("win_message:") => win_message = parse_string_param(w),
        c if c.starts_with('-') || c.starts_with('*') => cells.push(parse_cell(c)),
        _ => {}
    });

    // Free cell always has to be in the middle (commented as such in the brat file template)
    let free_cell_index = (size * size) / 2;
    Bingo::new(
        &title,
        &win_message,
        size,
        cells,
        (free_cell_index / size, free_cell_index % size),
    )
}

fn parse_size(size_line: &str) -> usize {
    size_line
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse size")
}

fn parse_string_param(param_line: &str) -> String {
    param_line.split_once(':').unwrap().1.trim().to_string()
}

fn parse_cell(cell_line: &str) -> Cell {
    cell_line
        .split_once(' ')
        .map(|(marker, label)| {
            let mut cell = Cell::new(label.trim());
            if marker == "*" {
                cell.toggle();
            }
            cell
        })
        .unwrap()
}
