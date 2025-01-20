use game::*;
use models::Cell;
use utils::print_cell_grid;

mod models;
mod game;
mod utils;

const CELLCOUNT: i64 = 9;
const GRIDSIZE: usize = 5;

fn main() {
    
    let mut cellarr: [[Cell; GRIDSIZE]; GRIDSIZE] = create_cell_array();
    loop {
        if print_cell_grid(cellarr) == -1 {
            break;
        }
        run(&mut cellarr);
    }    
}
