use std::{collections::HashMap, io::{self, Write}};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor,
    style::{self, Color, Stylize},
    event::{self, Event, KeyCode, KeyEvent},
};

enum CellType {
    Wall,
}

struct Map {
    cells: HashMap<(u16, u16), CellType>,
}

impl Map {
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    fn add_cell(&mut self, x: u16, y: u16, cell: CellType) {
        self.cells.insert((x, y), cell);
    }
}

fn draw_game(stdout: &mut io::Stdout, map: &Map) -> io::Result<()> {
    for cell in map.cells.iter() {
            stdout
                    .queue(cursor::MoveTo(cell.0.0, cell.0.1))?
                    .queue(style::PrintStyledContent(
                        style::StyledContent::new(
                            style::ContentStyle::new().with(Color::White),
                            "|||",
                        ),
                    ))?;
    }

    stdout.flush()?;
    Ok(())
}

fn generate_map(map: &mut Map, grid_size_x: u16, grid_size_y: u16) {
    for x in 0..=grid_size_x {
        map.add_cell(x, 0, CellType::Wall);
        map.add_cell(x, grid_size_y, CellType::Wall);
    }
    
    for y in 1..grid_size_y {
        map.add_cell(0, y, CellType::Wall);
        map.add_cell(grid_size_x, y, CellType::Wall);
    }
}

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
