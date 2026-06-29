use color_eyre::eyre::{Context, Error, Ok, Result};
use std::{fs, path::PathBuf};

use crate::app::{Bingo, cell::Cell};

pub fn parse_predictions(path: &str) -> Result<Bingo, Error> {
    let file = PathBuf::from(path);
    let contents = fs::read_to_string(file)?;

    let mut title: Option<String> = None;
    let mut win_message: Option<String> = None;
    let mut size: Option<usize> = None;
    let mut cells: Vec<Cell> = Vec::new();

    for (idx, line) in contents.lines().into_iter().enumerate() {
        let ln = idx + 1;
        match line {
            s if s.starts_with("size:") => {
                size = Some(parse_size(s).wrap_err(format!("Line {ln}:"))?)
            }
            t if t.starts_with("title:") => {
                title = Some(parse_string_param(t).wrap_err(format!("Line {ln}:"))?)
            }
            w if w.starts_with("win_message:") => {
                win_message = Some(parse_string_param(w).wrap_err(format!("Line {ln}:"))?)
            }
            c if c.starts_with('-') || c.starts_with('*') => {
                cells.push(parse_cell(c).wrap_err(format!("Line {ln}:"))?)
            }
            _ => {
                // Ignore comments and anything else not supported
            }
        }
    }

    let size = size.ok_or_else(|| Error::msg("Size not specified in the input file!"))?;
    validate_size(&size)?;
    let title = title.ok_or_else(|| Error::msg("Title not specified in the input file!"))?;
    let win_message =
        win_message.ok_or_else(|| Error::msg("Win message not specified in the input file!"))?;
    validate_cells(&cells, &size)?;

    // Free cell always has to be in the middle (commented as such in the brat file template)
    let free_cell_index = (size * size) / 2;
    Ok(Bingo::new(
        &title,
        &win_message,
        size,
        cells,
        (free_cell_index, free_cell_index),
    ))
}

fn parse_size(size_line: &str) -> Result<usize, Error> {
    size_line
        .split_once(':')
        .ok_or_else(|| Error::msg("Invalid size line format!"))
        .and_then(|(_, size_str)| {
            size_str
                .trim()
                .parse::<usize>()
                .map_err(|_| Error::msg("Size is not a valid number!"))
        })
}

fn parse_string_param(param_line: &str) -> Result<String, Error> {
    Ok(param_line
        .split_once(':')
        .ok_or_else(|| Error::msg("Invalid param line format!"))?
        .1
        .trim()
        .to_string())
}

fn parse_cell(cell_line: &str) -> Result<Cell, Error> {
    cell_line
        .split_once(' ')
        .ok_or_else(|| Error::msg("Invalid cell line format!"))
        .map(|(marker, cell_text)| {
            let mut cell = Cell::new(cell_text.trim());
            if marker.trim() == "*" {
                cell.toggle();
            }
            cell
        })
}

fn validate_size(size: &usize) -> Result<(), Error> {
    // So far we only support 3 or 5 to properly fit into terminal, other sizes maybe some other day
    if !(*size == 3 || *size == 5) {
        Err(Error::msg("Size must be either 3 or 5!"))
    } else {
        Ok(())
    }
}

fn validate_cells(cells: &Vec<Cell>, size: &usize) -> Result<(), Error> {
    // Check we have correct nubmer of cells for specified grid size
    if cells.len() != size * size {
        return Err(Error::msg(format!(
            "Number of cells ({}) does not match the specified size ({}x{})!",
            cells.len(),
            size,
            size
        )));
    };

    // Check that free cell is in the middle
    let free_cell_index = (size * size) / 2;
    if cells[free_cell_index].label() != "FREE" {
        return Err(Error::msg("Free cell is not in the middle of the grid!"));
    }

    Ok(())
}
