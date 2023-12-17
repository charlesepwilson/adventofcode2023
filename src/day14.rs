use crate::utils::Solves;
use std::collections::HashMap;

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 14;
    type ParsedInput = Vec<Vec<u8>>; // by column rather than row
    type Output = usize;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        let mut result = Vec::new();
        for line in Self::read_file(dir) {
            for (i, &c) in line.as_bytes().iter().enumerate() {
                if result.len() <= i {
                    result.push(Vec::new());
                }
                result[i].push(c);
            }
        }
        result
    }

    fn part1(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        calculate_load_slide_up_grid(input)
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        let final_grid = do_cycles(input, 1000000000);
        calculate_load(&final_grid)
    }
}

const ROUND: u8 = b'O';
const FIXED: u8 = b'#';
const BLANK: u8 = b'.';

fn calculate_load_slide_up(column: Vec<u8>) -> usize {
    let len = column.len() as isize;
    let mut total_load = 0;
    let mut last_fixed = -1;
    for (i, &item) in column.iter().enumerate() {
        if item == FIXED {
            last_fixed = i as isize;
        } else if item == ROUND {
            last_fixed += 1;
            total_load += len - last_fixed;
        }
    }
    total_load as usize
}

fn calculate_load_slide_up_grid(grid: Vec<Vec<u8>>) -> usize {
    let mut total = 0;
    for col in grid {
        total += calculate_load_slide_up(col);
    }
    total
}

fn roll_up(column: Vec<u8>) -> Vec<u8> {
    let mut last_fixed = -1;
    let mut new_column = vec![BLANK; column.len()];
    for (i, &item) in column.iter().enumerate() {
        if item == FIXED {
            last_fixed = i as isize;
            new_column[i] = item;
        } else if item == ROUND {
            last_fixed += 1;
            new_column[last_fixed as usize] = ROUND;
        }
    }
    new_column
}

fn roll_up_grid(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new_grid = Vec::new();
    for col in grid {
        new_grid.push(roll_up(col));
    }
    new_grid
}

fn rotate_clockwise(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let num_cols = grid.len();
    let num_rows = grid[0].len();
    let mut new_grid = vec![vec![BLANK; num_cols]; num_rows];
    for (i, col) in grid.iter().enumerate() {
        for (j, &item) in col.iter().enumerate() {
            new_grid[num_rows - j - 1][i] = item;
        }
    }
    new_grid
}

fn rotation_cycle(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new_grid = grid;
    for _ in 0..4 {
        new_grid = roll_up_grid(new_grid);
        new_grid = rotate_clockwise(new_grid);
    }
    new_grid
}

fn do_cycles(mut grid: Vec<Vec<u8>>, num_cycles: usize) -> Vec<Vec<u8>> {
    let mut grid_cache = HashMap::new();
    for i in 0..num_cycles {
        let grid_key = grid.clone();
        if let Some(&previous_iteration) = grid_cache.get(&grid_key) {
            let remaining_cycles = num_cycles - i;
            let functionally_remaining_cycles = remaining_cycles % (i - previous_iteration);
            return do_cycles(grid, functionally_remaining_cycles);
        } else {
            grid = rotation_cycle(grid);
            grid_cache.insert(grid_key, i);
        }
    }
    grid
}

fn calculate_load(grid: &Vec<Vec<u8>>) -> usize {
    let mut total_load = 0;
    let rows = grid[0].len();
    for col in grid {
        for (i, &item) in col.iter().enumerate() {
            if item == ROUND {
                total_load += rows - i
            }
        }
    }
    total_load
}
