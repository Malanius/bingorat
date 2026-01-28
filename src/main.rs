use color_eyre::eyre::Result;
use std::{error::Error, io};

use ratatui::{
    DefaultTerminal, Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    init, restore,
};

use crate::app::App;
use crate::ui::ui;

mod app;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;
    let mut terminal = init();

    let mut app = App::new();
    let _ = run_app(&mut terminal, &mut app);

    restore();
    terminal.show_cursor()?;

    Ok(())
}

fn run_app(terminal: &mut DefaultTerminal, app: &App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                _ => {}
            }
        }
    }
}
