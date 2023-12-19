use crate::utils::Solves;

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 18;
    type ParsedInput = Vec<((Direction, u32), (Direction, u32))>;
    type Output = usize;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        let mut result = Vec::new();
        for line in Self::read_file(dir) {
            let (dir, oth) = line.split_once(" ").unwrap();
            let (dist, mut hex) = oth.split_once(" ").unwrap();
            let direction = Direction::from_char(dir.chars().next().unwrap());
            let distance: u32 = dist.parse().unwrap();
            hex = hex.trim_matches('(').trim_matches(')').trim_matches('#');
            let hex_dir = Direction::from_num(hex.chars().last().unwrap().to_digit(10).unwrap());
            let hex_dist = u32::from_str_radix(&hex[..(hex.len()-2)], 16).unwrap();
            result.push(((direction, distance), (hex_dir, hex_dist)));
        }
        result
    }

    fn part1(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        calc_volume(input, false)
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        calc_volume(input, true)
    }
}

fn calc_volume(input: Vec<((Direction, u32), (Direction, u32))>, use_colour: bool) -> usize {
    let instructions = input.into_iter().map(|x| if use_colour{x.1} else {x.0}).collect();
    let mut dig_pattern = create_dig_pattern(&instructions);
    print_dig_pattern(&dig_pattern);
    dig_pattern = fill_in_dig_pattern(dig_pattern);
    print_dig_pattern(&dig_pattern);
    dig_pattern.into_iter().map(|x| x.into_iter().filter(|y| *y).count()).sum()
}

fn create_dig_pattern(instructions: &Vec<(Direction, u32)>) -> Vec<Vec<bool>> {
    let mut dug_out: Vec<(isize, isize)> = Vec::new();
    let mut current_coords = (0, 0);
    dug_out.push(current_coords);
    for (direction, distance) in instructions {
        for _ in 0..*distance {
            let (col_change, row_change) = direction.coordinate_change();
            current_coords.0 += row_change;
            current_coords.1 += col_change;
            dug_out.push(current_coords);
        }
    }
    let min_row = dug_out.iter().min_by_key(|&x| x.0).unwrap().0;
    let min_col = dug_out.iter().min_by_key(|&x| x.1).unwrap().1;
    for (r, c) in dug_out.iter_mut() {
        *r -= min_row;
        *c -= min_col;
    }
    let max_row = dug_out.iter().max_by_key(|&x| x.0).unwrap().0 as usize;
    let max_col = dug_out.iter().max_by_key(|&x| x.1).unwrap().1 as usize;
    let mut dig_map = vec![vec![false;max_col+1];max_row+1];
    for (r, c) in dug_out {
        dig_map[r as usize][c as usize] = true;
    }
    dig_map
}

fn fill_in_dig_pattern(pattern: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_pattern = pattern.clone();
    for (i, row) in pattern.iter().enumerate() {
        let mut inside = false;
        for (j, item) in row.iter().enumerate() {
            if *item {
                let block = pattern[i][j..].iter().take_while(|x| **x);
                let num_dug = block.count();
                if let Some(row_below) = pattern.get(i+1) {
                    if let Some(row_above_index) = i.checked_sub(1) {
                        if (!pattern[row_above_index][j]) && (!row_below[j]) {continue;}
                        let entered_from_below = row_below[j];
                        let exit_to_above = pattern[row_above_index][j+num_dug-1];
                        if entered_from_below == exit_to_above {inside = !inside;}
                    }
                }
            }
            else {
                new_pattern[i][j] = inside;
            }
        }
    }
    new_pattern
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }

    fn from_num(c: u32) -> Self {
        match c {
            3 => Self::Up,
            1 => Self::Down,
            2 => Self::Left,
            0 => Self::Right,
            _ => panic!(),
        }
    }
}


impl Direction {
    fn coordinate_change(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

fn print_dig_pattern(p: &Vec<Vec<bool>>) {
    // println!();
    // for row in p.iter() {
    //     for item in row.iter() {
    //         print!("{}", if *item {'#'} else {'.'});
    //     }
    //     println!();
    // }
}
