mod models;
mod game;

use models::{Map};
use game::{draw_game,generate_map};
use std::{io::{self}};
use crossterm::{
    ExecutableCommand,
    terminal,
    event::{self, Event, KeyCode, KeyEvent},
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let mut game_map = Map::new();

    generate_map(&mut game_map, 100, 20);

    terminal::enable_raw_mode()?;

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    draw_game(&mut stdout, &game_map)?;

    loop {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
