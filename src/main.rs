mod models;
mod game;
mod generator;

use models::{Game};
use game::{draw_game,generate_map};
use std::{io::{self}};
use crossterm::{
    ExecutableCommand, cursor, event::{self, Event, KeyCode, KeyEvent}, terminal
};

use crate::game::{clear_game, move_player};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let mut game = Game::new();
    game.size_x = 74;
    game.size_y = 24;
    game.player_position = (1, 1);
    game.exit_position = (game.size_x - 1, game.size_y - 1);

    generate_map(&mut game);

    terminal::enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    draw_game(&mut stdout, &game, true)?;

    loop {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char('q') => {
                    clear_game(&mut stdout)?;
                    stdout.execute(cursor::Show)?;  
                    break
                },
                KeyCode::Left | KeyCode::Char('a') => move_player(&mut stdout, &mut game, models::MoveDirection::Left)?,
                KeyCode::Right | KeyCode::Char('d')  => move_player(&mut stdout, &mut game, models::MoveDirection::Right)?,
                KeyCode::Down | KeyCode::Char('s')  => move_player(&mut stdout, &mut game, models::MoveDirection::Bottom)?,
                KeyCode::Up | KeyCode::Char('w')  => move_player(&mut stdout, &mut game, models::MoveDirection::Top)?,
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    stdout.execute(cursor::Show)?;

    Ok(())
}
