use std::io;
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
    
    let mut cont = String::new();

    print_cell_grid(cellarr);
    io::stdin()
        .read_line(&mut cont)
        .expect("Failed to read line");
    gameloop(&mut cellarr);
    print_cell_grid(cellarr);
}
