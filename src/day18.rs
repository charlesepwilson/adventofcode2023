use crate::utils::Solves;

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 18;
    type ParsedInput = (Vec<(Direction, u32)>, Vec<(Direction, u32)>);
    type Output = i64;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        let mut result1 = Vec::new();
        let mut result2 = Vec::new();

        for line in Self::read_file(dir) {
            let (dir, oth) = line.split_once(" ").unwrap();
            let (dist, mut hex) = oth.split_once(" ").unwrap();
            let direction = Direction::from_char(dir.chars().next().unwrap());
            let distance: u32 = dist.parse().unwrap();
            hex = hex.trim_matches('(').trim_matches(')').trim_matches('#');
            let hex_dir = Direction::from_num(hex.chars().last().unwrap().to_digit(10).unwrap());
            let hex_dist = u32::from_str_radix(&hex[..(hex.len() - 1)], 16).unwrap();
            result2.push((hex_dir, hex_dist));
            result1.push((direction, distance));
        }
        (result1, result2)
    }

    fn part1(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        compute_area_from_instructions(input.0)
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        compute_area_from_instructions(input.1)
    }
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
    fn coordinate_change(&self) -> [i64; 2] {
        match self {
            Self::Up => [0, -1],
            Self::Down => [0, 1],
            Self::Left => [-1, 0],
            Self::Right => [1, 0],
        }
    }
}

fn find_vertices(instructions: &Vec<(Direction, u32)>) -> Vec<[i64; 2]> {
    let mut result = Vec::new();
    let mut current_coords = [0, 0];
    for (direction, distance) in instructions {
        let changes = direction.coordinate_change();
        for (i, v) in changes
            .into_iter()
            .zip(current_coords)
            .map(|(l, r)| (l * (*distance as i64)) + r)
            .enumerate()
        {
            current_coords[i] = v;
        }
        result.push(current_coords);
    }
    result
}

fn compute_area_from_instructions(instructions: Vec<(Direction, u32)>) -> i64 {
    let coordinates = find_vertices(&instructions);
    let internal_area = compute_internal_area(coordinates);
    let mut external_area = 0;
    for (_, distance) in instructions {
        external_area += distance;
    }
    external_area = external_area / 2;
    internal_area.saturating_add_unsigned(external_area as u64) + 1
}

fn compute_internal_area(coordinates: Vec<[i64; 2]>) -> i64 {
    let mut total = 0;
    let n = coordinates.len();
    for i in 0..n {
        let next_i = (i + 1) % n;
        total += coordinates[i][0] * coordinates[next_i][1];
        total -= coordinates[i][1] * coordinates[next_i][0];
    }
    total / 2
}
