use crate::{GAME_DEFAULT_SIZE_X, GAME_DEFAULT_SIZE_Y, generator::{init_map, make_paths}, models::{CellType, Game, MoveDirection, Point}};
use std::io::{self, Write};
use crossterm::{
    QueueableCommand, cursor, event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, style::{self, Color, Stylize}, terminal::{Clear, ClearType}
};

pub fn create_game(width: u16, height: u16) -> Game {
    
    let mut game = Game::new();
    game.size_x = width;
    game.size_y = height;
    game.player_position = Point::new(1, 1);
    game.exit_position = Point::new(game.size_x - 1, game.size_y - 1);

    generate_map(&mut game);

    game
}

pub fn draw_game(stdout: &mut io::Stdout, game: &Game, redraw_cells: bool) -> io::Result<()> {
    if redraw_cells {
        for (point, cell_type) in game.map.cells.iter() {
            match cell_type {
                CellType::Wall => draw_char(stdout, point.clone(), '█', Color::White)?,
                CellType::Empty => draw_char(stdout, point.clone(), ' ', Color::Black)?,
            }
        }
    }

    if game.player_position == game.exit_position {
        draw_char(stdout, game.player_position, 'W', Color::White)?;
    } else {
        draw_char(stdout, game.player_position, 'P', Color::Red)?;
        draw_char(stdout, game.exit_position, 'E', Color::White)?;
    }

    let status_bar = format!("Controls: WASD / arrows to move, Q to quit, N to create new game | moves: {}", game.total_moves);

    draw_line(stdout, game.size_y + 2, status_bar, Color::White)?;

    stdout.flush()?;
    Ok(())
}

fn draw_char(stdout: &mut io::Stdout, position: Point, char: char, color: Color) -> io::Result<()> {
    stdout
        .queue(cursor::MoveTo(position.x, position.y))?
        .queue(style::PrintStyledContent(
            style::StyledContent::new(
                style::ContentStyle::new().with(color),
                char,
            ),
        ))?;

    Ok(())
}

fn draw_line(stdout: &mut io::Stdout, line: u16, content: String, color: Color) -> io::Result<()> {
    stdout
        .queue(cursor::MoveTo(0, line))?
        .queue(Clear(ClearType::CurrentLine))?
        .queue(style::PrintStyledContent(
            style::StyledContent::new(
                style::ContentStyle::new().with(color),
                content,
            ),
        ))?;

    Ok(())
}

pub fn clear_line(stdout: &mut io::Stdout, line: u16) -> io::Result<()> {
    stdout
        .queue(cursor::MoveTo(0, line))?
        .queue(Clear(ClearType::CurrentLine))?;

    Ok(())
}

pub fn generate_map(game: &mut Game) {
    init_map(game);
    make_paths(game);
}

pub fn move_player(stdout: &mut io::Stdout, game: &mut Game, direction: MoveDirection) -> io::Result<()> {
    let mut new_position = game.player_position;
    
    match direction {
        MoveDirection::Left => new_position.x -= 1,
        MoveDirection::Right => new_position.x += 1,
        MoveDirection::Bottom => new_position.y += 1,
        MoveDirection::Top => new_position.y -= 1,
    }
    
    if !can_move(new_position, game) {
        return Ok(())
    }

    game.total_moves += 1;

    clear_player_position(stdout, game.player_position)?;

    game.player_position = new_position;

    draw_game(stdout, game, false)?;

    Ok(())
}

fn clear_player_position(stdout: &mut io::Stdout, position: Point) -> io::Result<()> {
    stdout
        .queue(cursor::MoveTo(position.x, position.y))?
        .queue(style::PrintStyledContent(
            style::StyledContent::new(
                style::ContentStyle::new().with(Color::White),
                " ",
            ),
        ))?;

    Ok(())
}

fn can_move(point: Point, game: &Game) -> bool {
    match game.map.cells.get(&point) {
        Some(CellType::Wall) => false,
        Some(CellType::Empty) => true,
        None => false,
    }
}

pub fn draw_input_prompt(stdout: &mut io::Stdout, game: &Game, buffer: &str) -> io::Result<()> {
    let content = format!("New grid size (default: {}x{}): {}_", GAME_DEFAULT_SIZE_X, GAME_DEFAULT_SIZE_Y, buffer);
    draw_line(stdout, game.size_y + 3, content, Color::Yellow)?;
    stdout.flush()?;
    Ok(())
}

pub fn prompt_for_new_game(stdout: &mut io::Stdout, game: &mut Game) -> io::Result<()> {
    let mut buffer = format!("{}x{}", GAME_DEFAULT_SIZE_X, GAME_DEFAULT_SIZE_Y);
    draw_input_prompt(stdout, game, &buffer)?;

    loop {
        if let Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. }) = event::read()? {
            match code {
                KeyCode::Char(c) => {
                    if matches!(c, '0'..='9' | 'x') {
                        buffer.push(c);
                        draw_input_prompt(stdout, game, &buffer)?;
                    }
                },
                KeyCode::Backspace => {
                    buffer.pop();
                    draw_input_prompt(stdout, game, &buffer)?;
                },
                KeyCode::Enter => break,
                _ => {}
            }
        }
    }

    let (width, height) = buffer.split_once('x')
        .map(|(w, h)| (w.parse().unwrap_or(GAME_DEFAULT_SIZE_X), h.parse().unwrap_or(GAME_DEFAULT_SIZE_Y)))
        .unwrap_or((0, 0));

    let width = if width % 2 != 0 { width + 1 } else { width };
    let height = if height % 2 != 0 { height + 1 } else { height };

    clear_line(stdout, game.size_y + 3)?;

    clear_game(stdout)?;

    *game = create_game(width, height);
    draw_game(stdout, game, true)?;

    Ok(())
}

pub fn clear_game(stdout: &mut io::Stdout) -> io::Result<()> {
    stdout.queue(Clear(ClearType::All))?;

    stdout.flush()?;

    Ok(())
}