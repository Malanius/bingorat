use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    macros::{constraint, constraints},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let title = Line::from(" Bingo of the year ".green().bold());
    let instructions = Line::from(vec![
        " ←↑↓→/hjkl ".blue().bold(),
        "move ━━".into(),
        " <Space/Enter> ".blue().bold(),
        "toggle ━━".into(),
        " <R> ".blue().bold(),
        "reset ━━".into(),
        " <Q> ".blue().bold(),
        "quit ".into(),
    ]);

    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(5)])
        .split(block.inner(frame.area()));

    let label_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let label_area = chunks[1];
    frame.render_widget(label_block, label_area);

    frame.render_widget(block, frame.area());

    let grid_area = chunks[0];
    render_grid_ratio(frame, grid_area, app);
}

pub fn render_grid_ratio(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .flex(Flex::Center)
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(area.height * 2),
            Constraint::Min(0),
        ])
        .split(area);
    let grid_area = chunks[1];

    let constraints = [Constraint::Ratio(1, app.grid_size() as u32)];

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints.clone().repeat(app.grid_size()))
        .split(grid_area);

    for (row_idx, row_area) in rows.iter().enumerate() {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints.clone().repeat(app.grid_size()))
            .split(*row_area);

        for (col_idx, col_area) in cols.iter().enumerate() {
            let index = row_idx * app.grid_size() + col_idx;
            if let Some(cell) = app.cells().get(index) {
                let mut cell_block = Block::default().borders(Borders::ALL);
                if app.cursor_position() == (col_idx, row_idx) {
                    cell_block = cell_block.border_style(Style::default().fg(Color::Yellow));
                }
                let mut cell_paragraph = Paragraph::new(Text::from(cell.label())).block(cell_block);
                if cell.marked() {
                    cell_paragraph = cell_paragraph.style(Style::default().bg(Color::Green));
                }
                frame.render_widget(cell_paragraph, *col_area);
            }
        }
    }
}
