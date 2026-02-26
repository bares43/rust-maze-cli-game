use crate::models::{CellType, Game,};
use rand::{seq::SliceRandom};

pub fn init_map(game: &mut Game) {
    for x in 0..=game.size_x {
        for y in 0..=game.size_y {
            game.map.add_cell(x, y, CellType::Wall);
        }
    }
}

pub fn make_paths(game: &mut Game) {
    let mut stack: Vec<(u16, u16)> = Vec::new();

    let mut current_position = game.player_position.clone();

    stack.push(current_position);

    game.map.cells.insert((current_position.0, current_position.1), CellType::Empty);

    let mut rng = rand::rng();
    
    while !stack.is_empty() {

        let directions = vec![(0, -2), (0, 2), (-2, 0), (2, 0)];
        let mut candidate_cells: Vec<(u16, u16)> = Vec::new();

        for dir in directions.iter() {
            let new_cell = (current_position.0.saturating_add_signed(dir.0), current_position.1.saturating_add_signed(dir.1));

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
    
        let cell_on_way_x = (current_position.0 as i32 + (chosen_cell.0 as i32 - current_position.0 as i32).signum()) as u16;
        let cell_on_way_y = (current_position.1 as i32 + (chosen_cell.1 as i32 - current_position.1 as i32).signum()) as u16;

        game.map.cells.insert((cell_on_way_x, cell_on_way_y), CellType::Empty);

        game.map.cells.insert((chosen_cell.0, chosen_cell.1), CellType::Empty);

        stack.push(chosen_cell);
        current_position = chosen_cell;
        
    }
}

fn can_move(new: (u16, u16), game: &Game) -> bool {
    if new.0 == 0 || new.1 == 0 {
        return false;
    }

    if new.0 >= game.size_x || new.1 >= game.size_y {
        return false;
    }

    match game.map.cells.get(&(new.0, new.1)) {
        Some(CellType::Wall) => true,
        Some(CellType::Empty) => false,
        None => false,
    }
}
