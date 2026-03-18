use crate::models::{CellType, Game, Point,};
use rand::{seq::SliceRandom};

pub fn init_map(game: &mut Game) {
    for x in 0..=game.size_x {
        for y in 0..=game.size_y {
            game.map.add_cell(Point::new(x, y), CellType::Wall);
        }
    }
}

pub fn make_paths(game: &mut Game) {
    let mut stack: Vec<Point> = Vec::new();

    let mut current_position = game.player_position.clone();

    stack.push(current_position);

    game.map.cells.insert(current_position, CellType::Empty);

    let mut rng = rand::rng();
    
    while !stack.is_empty() {

        let directions = vec![(0, -2), (0, 2), (-2, 0), (2, 0)];
        let mut candidate_cells: Vec<Point> = Vec::new();

        for dir in directions.iter() {
            let new_cell = Point::new(current_position.x.saturating_add_signed(dir.0), current_position.y.saturating_add_signed(dir.1));

            if can_move(new_cell, game) {
                candidate_cells.push(new_cell);
            }
        }

        if candidate_cells.is_empty() {
            stack.pop();
            
            if stack.is_empty() {
                break;
            }

            if let Some(&last_item) = stack.last() {
                current_position = last_item;
            }

            continue;
        }

        candidate_cells.shuffle(&mut rng);

        let chosen_cell = candidate_cells[0];
    
        let cell_on_way_x = (current_position.x as i32 + (chosen_cell.x as i32 - current_position.x as i32).signum()) as u16;
        let cell_on_way_y = (current_position.y as i32 + (chosen_cell.y as i32 - current_position.y as i32).signum()) as u16;

        game.map.cells.insert(Point::new(cell_on_way_x, cell_on_way_y), CellType::Empty);

        game.map.cells.insert(Point::new(chosen_cell.x, chosen_cell.y), CellType::Empty);

        stack.push(chosen_cell);
        current_position = chosen_cell;
        
    }
}

fn can_move(new: Point, game: &Game) -> bool {
    if new.x == 0 || new.y == 0 {
        return false;
    }

    if new.x >= game.size_x || new.y >= game.size_y {
        return false;
    }

    match game.map.cells.get(&Point::new(new.x, new.y)) {
        Some(CellType::Wall) => true,
        Some(CellType::Empty) => false,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_game() -> Game {
        let mut game = Game::new();
        game.size_x = 10;
        game.size_y = 10;
        game
    }

    // Boundary checks
    #[test]
    fn can_move_boundary_x_is_zero() {
        let game = make_game();
        assert_eq!(can_move(Point::new(0, 5), &game), false);
    }

    #[test]
    fn can_move_boundary_y_is_zero() {
        let game = make_game();
        assert_eq!(can_move(Point::new(5, 0), &game), false);
    }

    #[test]
    fn can_move_boundary_x_exceeds_size() {
        let game = make_game();
        assert_eq!(can_move(Point::new(10, 5), &game), false);
    }

    #[test]
    fn can_move_boundary_y_exceeds_size() {
        let game = make_game();
        assert_eq!(can_move(Point::new(5, 10), &game), false);
    }

    // Cell type checks
    #[test]
    fn can_move_wall_cell_returns_true() {
        let mut game = make_game();
        game.map.add_cell(Point::new(5, 5), CellType::Wall);
        assert_eq!(can_move(Point::new(5, 5), &game), true);
    }

    #[test]
    fn can_move_empty_cell_returns_false() {
        let mut game = make_game();
        game.map.add_cell(Point::new(5, 5), CellType::Empty);
        assert_eq!(can_move(Point::new(5, 5), &game), false);
    }

    // Missing cell check
    #[test]
    fn can_move_missing_cell_returns_false() {
        let game = make_game();
        assert_eq!(can_move(Point::new(5, 5), &game), false);
    }
}