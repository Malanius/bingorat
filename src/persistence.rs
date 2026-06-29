use std::{fs, path::PathBuf};

use crate::app::{Bingo, cell::Cell};

pub fn parse_predictions(path: &str) -> Bingo {
    let file = PathBuf::from(path);
    let contents = fs::read_to_string(file).expect("Failed to read file");

    let filtered_contents: String = contents
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .join("\n");

    // println!("File contents:\n{filtered_contents}");

    // let output_file = PathBuf::from("filtered_predictions.brat");
    // fs::write(&output_file, filtered_contents).expect("Failed to write file");
    // println!("Filtered contents written to: {}", output_file.display());

    let mut title = String::new();
    let mut win_message = String::new();
    let mut size = 0;
    let mut cells: Vec<Cell> = Vec::new();

    filtered_contents.split('\n').for_each(|line| match line {
        l if line.starts_with("size:") => size = parse_size(l),
        l if line.starts_with("name:") => title = parse_string_param(l),
        l if line.starts_with("win_message:") => win_message = parse_string_param(l),
        l if line.starts_with('-') => cells.push(parse_cell(l)),
        l if line.starts_with('*') => cells.push(parse_cell(l)),
        _ => {}
    });

    // Free cell always has to be in the middle (ommented as such in the brat file template)
    let free_cell_index = (size * size) / 2;
    Bingo::new(&title, &win_message, size, cells, (free_cell_index / size, free_cell_index % size)) 
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
    param_line.split(':').nth(1).unwrap().trim().to_string()
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
