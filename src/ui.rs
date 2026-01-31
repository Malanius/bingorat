use std::vec;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
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
        .constraints([Constraint::Min(0), Constraint::Length(4)])
        .split(block.inner(frame.area()));

    frame.render_widget(block, frame.area());

    let grid_area = chunks[0];
    render_grid(frame, grid_area, app);

    let label_area = chunks[1];
    render_cell_label(frame, label_area, app);
}

pub fn render_grid(frame: &mut Frame, area: Rect, app: &App) {
    let grid_area_layout = Layout::default()
        .flex(Flex::Center)
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(area.height * 2),
            Constraint::Min(0),
        ])
        .split(area);
    let grid_area = grid_area_layout[1];

    let constraints = [Constraint::Ratio(1, app.grid_size() as u32)].repeat(app.grid_size());
    let horizontal = Layout::horizontal(constraints.clone());
    let vertical = Layout::vertical(constraints.clone());

    let rows = vertical.split(grid_area);
    let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

    for (idx, cell_area) in cells.into_iter().enumerate() {
        if let Some(cell) = app.cells().get(idx) {
            let mut cell_block = Block::default().borders(Borders::ALL);
            if app.cursor_position() == (idx % app.grid_size(), idx / app.grid_size()) {
                cell_block = cell_block.border_style(Style::default().fg(Color::Yellow));
            }
            let mut cell_paragraph = Paragraph::new(Text::from(cell.label())).block(cell_block);
            if cell.marked() {
                cell_paragraph = cell_paragraph.style(Style::default().bg(Color::Green));
            }
            frame.render_widget(cell_paragraph, cell_area);
        }
    }
}

pub fn render_cell_label(frame: &mut Frame, area: Rect, app: &App) {
    let current_index: usize = app.current_index();
    let cell = match app.current_cell() {
        Some(c) => c,
        None => return,
    };

    let title = Line::from(vec![
        " Current Cell: ".blue().bold(),
        current_index.yellow().bold(),
        " (Marked: ".into(),
        if cell.marked() {
            "Yes".green().bold()
        } else {
            "No".red().bold()
        },
        ")".into(),
    ]);

    let label_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .title(title.centered());

    let label_paragraph = Paragraph::new(Text::from(cell.label()))
        .block(label_block.clone())
        .alignment(Alignment::Center);

    frame.render_widget(label_block, area);
    frame.render_widget(label_paragraph, area);
}
