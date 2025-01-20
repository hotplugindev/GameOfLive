use crate::models::*;
use crate::{GRIDSIZE, CELLCOUNT};
use rand::{thread_rng, Rng};

pub fn create_cell_array() -> [[Cell; GRIDSIZE]; GRIDSIZE] {
    let mut rng = thread_rng();
    let mut cellarr: [[Cell; GRIDSIZE]; GRIDSIZE] = [[
        Cell{ is_alive: false , next_tick_alive: false }; GRIDSIZE
    ]; GRIDSIZE];

    for _i in 0..CELLCOUNT {
        cellarr[rng.gen_range(0..GRIDSIZE)][rng.gen_range(0..GRIDSIZE)].is_alive = true;
    }
    
    cellarr
}

fn get_cell_neighbor_count(cellarr: [[Cell; GRIDSIZE]; GRIDSIZE], i: usize, j: usize) -> i8 {
    let mut cellcount: i8 = 0;
    let i: i32 = i as i32;
    let j: i32 = j as i32;

    for i2 in i-1..i+1{
        if i2 as usize > GRIDSIZE || i2 < 0 { continue; }
        for j2 in j-1..j+1{
            if j2 as usize > GRIDSIZE || j2 < 0 { continue; } 
            if i == i2 && j == j2 { continue; }
            if cellarr[i2 as usize][j2 as usize].is_alive {
                cellcount += 1;
            }
        }
    }
    cellcount
}

fn apply_next_tick(cellarr: &mut [[Cell; GRIDSIZE]; GRIDSIZE]){
    for i in 0..GRIDSIZE{
        for j in 0.. GRIDSIZE{
            cellarr[i][j].is_alive = cellarr[i][j].next_tick_alive;
        }
    }
}

pub fn run(cellarr: &mut [[Cell; GRIDSIZE]; GRIDSIZE]){
    for i in 0..GRIDSIZE{
        for j in 0..GRIDSIZE{
            match get_cell_neighbor_count(*cellarr, i, j) {
                2 => if cellarr[i][j].is_alive { cellarr[i][j].next_tick_alive = true; } else { cellarr[i][j].next_tick_alive = false; },
                3 => cellarr[i][j].next_tick_alive = true,
                _ => cellarr[i][j].next_tick_alive = false,
            }
        }
    }
    apply_next_tick(cellarr);
}
