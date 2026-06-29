use color_eyre::eyre::{Ok, Result};

use ratatui::{
    DefaultTerminal,
    crossterm::{
        event::{self, Event, KeyEventKind},
        terminal::enable_raw_mode,
    },
    init, restore,
};

use crate::app::App;
use crate::input::Action;
use crate::ui::ui;

mod app;
mod input;
mod ui;
mod y2026;
mod persistence;

fn main() -> Result<()> {
    color_eyre::install()?;

    // let bingo = y2026::get_year_predictions();
    let bingo = persistence::parse_predictions("2026-predictions.brat");
    enable_raw_mode()?;
    let mut terminal = init();

    let app = App::from(&bingo);
    run_app(&mut terminal, app)?;

    restore();
    terminal.show_cursor()?;

    Ok(())
}

fn run_app(terminal: &mut DefaultTerminal, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            if app.game_won() {
                match Action::from_key(key) {
                    Action::Reset => app.reset(),
                    Action::Quit => return Ok(()),
                    _ => {}
                }
            } else {
                match Action::from_key(key) {
                    Action::MoveCursor(direction) => app.move_cursor(&direction),
                    Action::Toggle => app.toggle_current_cell(),
                    Action::Reset => app.reset(),
                    Action::Quit => return Ok(()),
                    Action::None => {}
                }
            }
        }
    }
}
