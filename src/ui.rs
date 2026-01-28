use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    style::{Color, Style},
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

    let grid_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let grid_area = chunks[0];
    frame.render_widget(grid_block, grid_area);

    let label_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let label_area = chunks[1];
    frame.render_widget(label_block, label_area);

    frame.render_widget(block, frame.area());
}
