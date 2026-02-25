use crate::models::{Map, CellType};
use std::io::{self, Write};
use crossterm::{
    QueueableCommand,
    cursor,
    style::{self, Color, Stylize},
};

pub fn draw_game(stdout: &mut io::Stdout, map: &Map) -> io::Result<()> {
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