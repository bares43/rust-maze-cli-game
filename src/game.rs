use crate::{generator::{init_map, make_paths}, models::{CellType, Game, MoveDirection}};
use std::io::{self, Write};
use crossterm::{
    QueueableCommand,
    cursor,
    style::{self, Color, Stylize}, terminal::{Clear, ClearType},
};

pub fn draw_game(stdout: &mut io::Stdout, game: &Game) -> io::Result<()> {
    for ((x, y), cell_type) in game.map.cells.iter() {

        match cell_type {
            CellType::Wall => draw_char(stdout, (*x, *y), '█', Color::White)?,
            CellType::Empty => draw_char(stdout, (*x, *y), ' ', Color::Black)?,
        }

    }

    if game.player_position == game.exit_position {
        draw_char(stdout, game.player_position, 'W', Color::White)?;
    } else {
        draw_char(stdout, game.player_position, 'P', Color::Red)?;
        draw_char(stdout, game.exit_position, 'E', Color::White)?;
    }

    stdout.flush()?;
    Ok(())
}

fn draw_char(stdout: &mut io::Stdout, position: (u16, u16), char: char, color: Color) -> io::Result<()> {
    stdout
        .queue(cursor::MoveTo(position.0, position.1))?
        .queue(style::PrintStyledContent(
            style::StyledContent::new(
                style::ContentStyle::new().with(color),
                char,
            ),
        ))?;

    Ok(())
}

pub fn generate_map(game: &mut Game) {
    init_map(game);
    make_paths(game);
}

pub fn move_player(stdout: &mut io::Stdout, game: &mut Game, direction: MoveDirection) -> io::Result<()> {
    let mut new_position = game.player_position.clone();
    
    match direction {
        MoveDirection::Left => new_position.0 -= 1,
        MoveDirection::Right => new_position.0 += 1,
        MoveDirection::Bottom => new_position.1 += 1,
        MoveDirection::Top => new_position.1 -= 1,
    }
    
    if !can_move(new_position.0, new_position.1, game) {
        return Ok(())
    }

    clear_player_position(stdout, &game.player_position)?;

    game.player_position = new_position;

    draw_game(stdout, game)?;

    Ok(())
}

fn clear_player_position(stdout: &mut io::Stdout, position: &(u16, u16)) -> io::Result<()> {
    stdout
        .queue(cursor::MoveTo(position.0, position.1))?
        .queue(style::PrintStyledContent(
            style::StyledContent::new(
                style::ContentStyle::new().with(Color::White),
                " ",
            ),
        ))?;

    Ok(())
}

fn can_move(x: u16, y: u16, game: &Game) -> bool {
    match game.map.cells.get(&(x, y)) {
        Some(CellType::Wall) => false,
        Some(CellType::Empty) => true,
        None => false,
    }
}

pub fn clear_game(stdout: &mut io::Stdout) -> io::Result<()> {
    stdout.queue(Clear(ClearType::All))?;

    stdout.flush()?;

    Ok(())
}