use crate::models::{CellType, Game, Map, MoveDirection};
use std::io::{self, Write};
use crossterm::{
    QueueableCommand,
    cursor,
    style::{self, Color, Stylize}, terminal::{Clear, ClearType},
};

pub fn draw_game(stdout: &mut io::Stdout, game: &Game) -> io::Result<()> {
    for cell in game.map.cells.iter() {
        draw_char(stdout, (cell.0.0, cell.0.1), '#')?;
    }

    if game.player_position == game.exit_position {
        draw_char(stdout, game.player_position, 'W')?;
    } else {
        draw_char(stdout, game.player_position, 'P')?;
        draw_char(stdout, game.exit_position, 'E')?;
    }

    stdout.flush()?;
    Ok(())
}

fn draw_char(stdout: &mut io::Stdout, position: (u16, u16), char: char) -> io::Result<()> {
    stdout
        .queue(cursor::MoveTo(position.0, position.1))?
        .queue(style::PrintStyledContent(
            style::StyledContent::new(
                style::ContentStyle::new().with(Color::White),
                char,
            ),
        ))?;

    Ok(())
}

pub fn generate_map(map: &mut Map, grid_size_x: u16, grid_size_y: u16) {
    for x in 0..=grid_size_x {
        map.add_cell(x, 0, CellType::Wall);
        map.add_cell(x, grid_size_y, CellType::Wall);
    }
    
    for y in 1..grid_size_y {
        map.add_cell(0, y, CellType::Wall);
        map.add_cell(grid_size_x, y, CellType::Wall);
    }
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
    !game.map.cells.contains_key(&(x, y))
}

pub fn clear_game(stdout: &mut io::Stdout) -> io::Result<()> {
    stdout.queue(Clear(ClearType::All))?;

    Ok(())
}