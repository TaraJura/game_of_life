use std::{thread, time::Duration};
use rand::Rng;

const WIDTH: usize = 60;
const HEIGHT: usize = 25;

fn main() {
    let mut grid = initialize_random_grid();

    loop {
        clear_screen();
        print_grid(&grid);
        grid = update_grid(&grid);
        thread::sleep(Duration::from_millis(100));
    }
}

fn initialize_random_grid() -> [[bool; WIDTH]; HEIGHT] {
    let mut rng = rand::thread_rng();
    let mut grid = [[false; WIDTH]; HEIGHT];
    
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell = rng.gen_bool(0.3); // 30% chance of a cell being alive
        }
    }

    grid
}

fn print_grid(grid: &[[bool; WIDTH]; HEIGHT]) {
    for row in grid.iter() {
        for &cell in row.iter() {
            print!("{}", if cell { "■" } else { "□" });
        }
        println!();
    }
}

fn update_grid(grid: &[[bool; WIDTH]; HEIGHT]) -> [[bool; WIDTH]; HEIGHT] {
    let mut new_grid = [[false; WIDTH]; HEIGHT];

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let live_neighbors = count_live_neighbors(grid, i, j);
            new_grid[i][j] = match (grid[i][j], live_neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    new_grid
}

fn count_live_neighbors(grid: &[[bool; WIDTH]; HEIGHT], row: usize, col: usize) -> u8 {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let new_row = (row as i32 + i).rem_euclid(HEIGHT as i32) as usize;
            let new_col = (col as i32 + j).rem_euclid(WIDTH as i32) as usize;
            if grid[new_row][new_col] {
                count += 1;
            }
        }
    }
    count
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}