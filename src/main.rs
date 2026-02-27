mod models;
mod game;
mod generator;

use models::{Game};
use game::{draw_game,generate_map};
use std::{io::{self}};
use crossterm::{
    ExecutableCommand, cursor, event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, terminal
};

use crate::game::{clear_game, create_game, move_player};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let mut game = create_game();

    terminal::enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    draw_game(&mut stdout, &game, true)?;

    loop {
        if let Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. }) = event::read()? {
            match code {
                KeyCode::Char('q') => {
                    clear_game(&mut stdout)?;
                    stdout.execute(cursor::Show)?;  
                    break
                },
                KeyCode::Char('n') => {
                    clear_game(&mut stdout)?;
                    game = create_game();
                    draw_game(&mut stdout, &game, true)?;
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
