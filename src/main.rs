mod models;
mod game;
mod generator;

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
    game.size_x = 55;
    game.size_y = 14;
    game.player_position = (1, 1);
    game.exit_position = (game.size_x - 1, game.size_y - 1);

    generate_map(&mut game);

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
