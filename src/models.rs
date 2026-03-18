use std::{collections::HashMap};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }
}

pub enum CellType {
    Wall,
    Empty,
}

pub struct Map {
    pub cells: HashMap<Point, CellType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    pub fn add_cell(&mut self, point: Point, cell: CellType) {
        self.cells.insert(point, cell);
    }
}

pub struct Game {
    pub map: Map,
    pub player_position: Point,
    pub exit_position: Point,
    pub size_x : u16,
    pub size_y: u16,
    pub total_moves: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            player_position: Point::zero(),
            exit_position: Point::zero(),
            size_x: 0,
            size_y: 0,
            total_moves: 0,
        }
    }
}

pub enum MoveDirection {
    Left,
    Right,
    Top,
    Bottom,
}