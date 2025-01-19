use crate::{GRIDSIZE, CELLCOUNT};
use crate::models::Cell;
use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::stdout;

pub fn print_cell_grid(cellarr: [[Cell; GRIDSIZE]; GRIDSIZE]){
    execute!(stdout(), Clear(ClearType::All)).unwrap();

    print!("|");
    for _j in 0..GRIDSIZE {
        print!("-|");
    }
    println!();

    for i in 0..GRIDSIZE{
        print!("|");
        for j in 0..GRIDSIZE{
            if cellarr[i][j].is_alive {
                print!("█|");
            } else {
                print!(" |");
            }
        }
        println!();
        print!("|");
        for _j in 0..GRIDSIZE {
            print!("-|");
        }
        println!();
    }
}

