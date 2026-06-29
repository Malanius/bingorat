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

#[cfg(test)]
mod tests {
    use super::*;

    // ===== parse_size tests =====

    #[test]
    fn test_parse_size_valid() {
        assert_eq!(parse_size("size: 3").unwrap(), 3);
        assert_eq!(parse_size("size: 5").unwrap(), 5);
        assert_eq!(parse_size("size:3").unwrap(), 3);
        assert_eq!(parse_size("size:  5  ").unwrap(), 5);
    }

    #[test]
    fn test_parse_size_invalid_format() {
        assert!(parse_size("size 3").is_err());
        assert!(parse_size("size").is_err());
        assert!(parse_size("3").is_err());
    }

    #[test]
    fn test_parse_size_invalid_number() {
        assert!(parse_size("size: abc").is_err());
        assert!(parse_size("size: 3.5").is_err());
        assert!(parse_size("size: -1").is_err());
    }

    // ===== parse_string_param tests =====

    #[test]
    fn test_parse_string_param_valid() {
        assert_eq!(parse_string_param("title: My Title").unwrap(), "My Title");
        assert_eq!(
            parse_string_param("win_message: Congratulations!").unwrap(),
            "Congratulations!"
        );
        assert_eq!(parse_string_param("title:NoSpace").unwrap(), "NoSpace");
        assert_eq!(
            parse_string_param("title:   Extra Spaces   ").unwrap(),
            "Extra Spaces"
        );
    }

    #[test]
    fn test_parse_string_param_empty_value() {
        assert_eq!(parse_string_param("title:").unwrap(), "");
        assert_eq!(parse_string_param("title:   ").unwrap(), "");
    }

    #[test]
    fn test_parse_string_param_invalid_format() {
        assert!(parse_string_param("title").is_err());
        assert!(parse_string_param("no_colon_here").is_err());
    }

    #[test]
    fn test_parse_string_param_with_special_chars() {
        assert_eq!(
            parse_string_param("title: My Title 123!@#").unwrap(),
            "My Title 123!@#"
        );
    }

    // ===== parse_cell tests =====

    #[test]
    fn test_parse_cell_unmarked() {
        let cell = parse_cell("- This is a cell").unwrap();
        assert_eq!(cell.label(), "This is a cell");
    }

    #[test]
    fn test_parse_cell_marked() {
        let cell = parse_cell("* This is a marked cell").unwrap();
        assert_eq!(cell.label(), "This is a marked cell");
    }

    #[test]
    fn test_parse_cell_with_multiple_spaces() {
        let cell = parse_cell("-   Cell with spaces").unwrap();
        assert_eq!(cell.label(), "Cell with spaces");
    }

    #[test]
    fn test_parse_cell_marked_with_spaces() {
        let cell = parse_cell("*   Marked with spaces").unwrap();
        assert_eq!(cell.label(), "Marked with spaces");
    }

    #[test]
    fn test_parse_cell_special_characters() {
        let cell = parse_cell("- Cell with 123!@#$%^&*()").unwrap();
        assert_eq!(cell.label(), "Cell with 123!@#$%^&*()");
    }

    #[test]
    fn test_parse_cell_invalid_format() {
        assert!(parse_cell("-").is_err());
        assert!(parse_cell("*").is_err());
    }

    // ===== validate_size tests =====

    #[test]
    fn test_validate_size_valid() {
        assert!(validate_size(&3).is_ok());
        assert!(validate_size(&5).is_ok());
    }

    #[test]
    fn test_validate_size_invalid() {
        assert!(validate_size(&1).is_err());
        assert!(validate_size(&2).is_err());
        assert!(validate_size(&4).is_err());
        assert!(validate_size(&7).is_err());
        assert!(validate_size(&0).is_err());
    }

    // ===== validate_cells tests =====

    #[test]
    fn test_validate_cells_correct_size_3x3() {
        let mut cells = Vec::new();
        for i in 0..9 {
            if i == 4 {
                cells.push(Cell::new("FREE"));
            } else {
                cells.push(Cell::new(&format!("Cell {}", i)));
            }
        }
        assert!(validate_cells(&cells, &3).is_ok());
    }

    #[test]
    fn test_validate_cells_correct_size_5x5() {
        let mut cells = Vec::new();
        for i in 0..25 {
            if i == 12 {
                cells.push(Cell::new("FREE"));
            } else {
                cells.push(Cell::new(&format!("Cell {}", i)));
            }
        }
        assert!(validate_cells(&cells, &5).is_ok());
    }

    #[test]
    fn test_validate_cells_incorrect_count() {
        let cells = vec![Cell::new("Cell 1"), Cell::new("Cell 2")];
        assert!(validate_cells(&cells, &3).is_err());
    }

    #[test]
    fn test_validate_cells_missing_free_cell() {
        let mut cells = Vec::new();
        for i in 0..9 {
            cells.push(Cell::new(&format!("Cell {}", i)));
        }
        assert!(validate_cells(&cells, &3).is_err());
    }

    #[test]
    fn test_validate_cells_free_cell_not_in_middle() {
        let mut cells = Vec::new();
        for i in 0..9 {
            if i == 0 {
                cells.push(Cell::new("FREE"));
            } else {
                cells.push(Cell::new(&format!("Cell {}", i)));
            }
        }
        assert!(validate_cells(&cells, &3).is_err());
    }

    #[test]
    fn test_validate_cells_free_cell_wrong_label() {
        let mut cells = Vec::new();
        for i in 0..9 {
            if i == 4 {
                cells.push(Cell::new("NotFree"));
            } else {
                cells.push(Cell::new(&format!("Cell {}", i)));
            }
        }
        assert!(validate_cells(&cells, &3).is_err());
    }

    // ===== Integration tests =====

    #[test]
    fn test_parse_predictions_valid_3x3() {
        let content = "title: Test Bingo\n\
            win_message: You won!\n\
            size: 3\n\
            - Cell 1\n\
            - Cell 2\n\
            - Cell 3\n\
            - Cell 4\n\
            * FREE\n\
            - Cell 6\n\
            - Cell 7\n\
            - Cell 8\n\
            - Cell 9";

        let temp_file = "test_input_3x3.brat";
        fs::write(temp_file, content).expect("Failed to write temp file");

        let result = parse_predictions(temp_file);
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let bingo = result.unwrap();
        assert_eq!(bingo.grid_size(), 3);
        assert_eq!(bingo.title(), "Test Bingo");
        assert_eq!(bingo.win_message(), "You won!");
    }

    #[test]
    fn test_parse_predictions_valid_5x5() {
        let mut content = String::from(
            "title: Big Bingo\n\
            win_message: Congratulations!\n\
            size: 5\n",
        );

        for i in 0..25 {
            if i == 12 {
                content.push_str("* FREE\n");
            } else {
                content.push_str(&format!("- Prediction {}\n", i + 1));
            }
        }

        let temp_file = "test_input_5x5.brat";
        fs::write(temp_file, &content).expect("Failed to write temp file");

        let result = parse_predictions(temp_file);
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
        let bingo = result.unwrap();
        assert_eq!(bingo.grid_size(), 5);
    }

    #[test]
    fn test_parse_predictions_missing_title() {
        let content = "win_message: You won!\n\
            size: 3\n\
            - Cell 1\n\
            - Cell 2\n\
            - Cell 3\n\
            - Cell 4\n\
            * FREE\n\
            - Cell 6\n\
            - Cell 7\n\
            - Cell 8\n\
            - Cell 9";

        let temp_file = "test_missing_title.brat";
        fs::write(temp_file, content).expect("Failed to write temp file");

        let result = parse_predictions(temp_file);
        let _ = fs::remove_file(temp_file);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_predictions_missing_win_message() {
        let content = "title: Test\n\
            size: 3\n\
            - Cell 1\n\
            - Cell 2\n\
            - Cell 3\n\
            - Cell 4\n\
            * FREE\n\
            - Cell 6\n\
            - Cell 7\n\
            - Cell 8\n\
            - Cell 9";

        let temp_file = "test_missing_message.brat";
        fs::write(temp_file, content).expect("Failed to write temp file");

        let result = parse_predictions(temp_file);
        let _ = fs::remove_file(temp_file);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_predictions_missing_size() {
        let content = "title: Test\n\
            win_message: You won!\n\
            - Cell 1\n\
            - Cell 2\n\
            - Cell 3\n\
            - Cell 4\n\
            * FREE\n\
            - Cell 6\n\
            - Cell 7\n\
            - Cell 8\n\
            - Cell 9";

        let temp_file = "test_missing_size.brat";
        fs::write(temp_file, content).expect("Failed to write temp file");

        let result = parse_predictions(temp_file);
        let _ = fs::remove_file(temp_file);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_predictions_wrong_cell_count() {
        let content = "title: Test\n\
            win_message: You won!\n\
            size: 3\n\
            - Cell 1\n\
            - Cell 2\n\
            - Cell 3";

        let temp_file = "test_wrong_count.brat";
        fs::write(temp_file, content).expect("Failed to write temp file");

        let result = parse_predictions(temp_file);
        let _ = fs::remove_file(temp_file);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_predictions_invalid_size() {
        let content = "title: Test\n\
            win_message: You won!\n\
            size: 7\n\
            - Cell 1\n\
            - Cell 2\n\
            - Cell 3\n\
            - Cell 4\n\
            * FREE\n\
            - Cell 6\n\
            - Cell 7\n\
            - Cell 8\n\
            - Cell 9";

        let temp_file = "test_invalid_size.brat";
        fs::write(temp_file, content).expect("Failed to write temp file");

        let result = parse_predictions(temp_file);
        let _ = fs::remove_file(temp_file);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_predictions_ignores_comments() {
        let content = "# This is a comment\n\
            title: Test Bingo\n\
            // Another comment style\n\
            win_message: You won!\n\
            size: 3\n\
            - Cell 1\n\
            - Cell 2\n\
            - Cell 3\n\
            - Cell 4\n\
            * FREE\n\
            - Cell 6\n\
            - Cell 7\n\
            - Cell 8\n\
            - Cell 9\n\
            # Final comment";

        let temp_file = "test_with_comments.brat";
        fs::write(temp_file, content).expect("Failed to write temp file");

        let result = parse_predictions(temp_file);
        let _ = fs::remove_file(temp_file);

        assert!(result.is_ok());
    }
}
