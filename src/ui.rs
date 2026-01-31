use std::vec;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::app::{App, cell::Cell};

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

    let main_block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK)
        .bg(Color::Black);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(main_block.inner(frame.area()));

    frame.render_widget(main_block, frame.area());

    let grid_area = chunks[0];
    render_grid(frame, grid_area, app);

    let label_hint_area = chunks[1];
    render_current_cell_hint(frame, label_hint_area, app);
}

fn render_grid(frame: &mut Frame, area: Rect, app: &App) {
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
            let selected = app.cursor_position() == (idx % app.grid_size(), idx / app.grid_size());
            render_cell(cell, selected, cell_area, frame);
        }
    }
}

fn render_cell(cell: &Cell, selected: bool, cell_area: Rect, frame: &mut Frame) {
    let mut cell_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::QuadrantOutside)
        .border_style(Style::default().black())
        .bg(Color::Gray);

    if cell.marked() {
        cell_block = cell_block.bg(Color::Green);
    }
    if selected {
        cell_block = cell_block
            .border_style(Style::default().yellow())
            .bg(Color::Yellow);
    }

    let cell_paragraph = Paragraph::new(Text::from(cell.label()).bold())
        .block(cell_block)
        .wrap(Wrap { trim: true })
        .centered();

    frame.render_widget(cell_paragraph, cell_area);
}

fn render_current_cell_hint(frame: &mut Frame, area: Rect, app: &App) {
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
        .centered();

    frame.render_widget(label_block, area);
    frame.render_widget(label_paragraph, area);
}
