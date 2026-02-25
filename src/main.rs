mod models;
mod game;

use models::{Game};
use game::{draw_game,generate_map};
use std::{io::{self}};
use crossterm::{
    ExecutableCommand,
    terminal,
    event::{self, Event, KeyCode, KeyEvent},
};

use crate::game::{clear_game, move_player};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let mut game = Game::new();
    game.player_position = (1, 1);
    game.exit_position = (99,19);

    generate_map(&mut game.map, 100, 20);

    terminal::enable_raw_mode()?;

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    draw_game(&mut stdout, &game)?;

    loop {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char('q') => {
                    clear_game(&mut stdout)?;
                    break
                },
                KeyCode::Left => move_player(&mut stdout, &mut game, models::MoveDirection::Left)?,
                KeyCode::Right => move_player(&mut stdout, &mut game, models::MoveDirection::Right)?,
                KeyCode::Down => move_player(&mut stdout, &mut game, models::MoveDirection::Bottom)?,
                KeyCode::Up => move_player(&mut stdout, &mut game, models::MoveDirection::Top)?,
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
