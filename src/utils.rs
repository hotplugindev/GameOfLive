use crate::GRIDSIZE;
use std::io;
use crate::models::Cell;
use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::stdout;

pub fn print_cell_grid(cellarr: [[Cell; GRIDSIZE]; GRIDSIZE]) -> i32 {
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
    let mut cont = String::new();
    io::stdin()
        .read_line(&mut cont)
        .expect("Failed to read line");
    if cont.trim() == "q" {
        return -1;
    }
    0
}

