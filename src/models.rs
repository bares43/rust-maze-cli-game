use std::{collections::HashMap};

pub enum CellType {
    Wall,
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