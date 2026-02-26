use std::{collections::HashMap};

pub enum CellType {
    Wall,
    Empty,
}

pub struct Map {
    pub cells: HashMap<(u16, u16), CellType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    pub fn add_cell(&mut self, x: u16, y: u16, cell: CellType) {
        self.cells.insert((x, y), cell);
    }
}

pub struct Game {
    pub map: Map,
    pub player_position: (u16, u16),
    pub exit_position: (u16, u16),
    pub size_x : u16,
    pub size_y: u16,
}

impl Game {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            player_position: (0,0),
            exit_position: (0, 0),
            size_x: 0,
            size_y: 0,
        }
    }
}

pub enum MoveDirection {
    Left,
    Right,
    Top,
    Bottom,
}