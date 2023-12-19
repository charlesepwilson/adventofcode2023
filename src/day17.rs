use std::collections::HashMap;
use crate::utils::Solves;

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 17;
    type ParsedInput = Vec<Vec<u32>>;
    type Output = u32;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        Self::read_file(dir)
            .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect())
            .collect()
    }

    fn part1(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        let num_rows = input.len();
        let num_cols = input[0].len();
        find_optimal_path((0, 0), (num_rows - 1, num_cols - 1), input)
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        0
    }
}

fn find_optimal_path(start: (usize, usize), dest: (usize, usize), grid: Vec<Vec<u32>>) -> u32 {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut record_grid = vec![vec![TraversalRecord::default(); num_cols]; num_rows];
    let (mut row, mut col) = start;
    let initial_path = Path{distance: 0, path: vec![], extended: false};
    record_grid[row][col].paths = HashMap::from([(initial_path.num_repeats(), initial_path)]);
    loop {
        let paths = record_grid[row][col].paths.clone();
        for ((last_direction, repeats), p) in paths {
            for d in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                const MAX_REPEATS: u32 = 3;
                if (repeats >= MAX_REPEATS) && (last_direction == d) { continue; }
                if d == last_direction.opposite() {continue;}


                let (col_change, row_change) = d.coordinate_change();
                if let (Some(new_row), Some(new_col)) = (row.checked_add_signed(row_change), col.checked_add_signed(col_change)) {
                    if (new_row < num_rows) && (new_col < num_cols) {
                        let mut new_path = p.path.clone();
                        new_path.push(d);
                        let new_distance = p.distance + grid[new_row][new_col];
                        let new_p = Path{distance: new_distance, path: new_path, extended: false};
                        let key = new_p.num_repeats();
                        let existing_p = record_grid[new_row][new_col].paths.get(&key);
                        if existing_p.is_none() || (existing_p.unwrap().distance > new_distance) {
                            record_grid[new_row][new_col].paths.insert(key, new_p);
                        }

                    }
                }
            }
        }
        for p in record_grid[row][col].paths.values_mut() {p.extended = true;}

        let mut min_tentative_dist = u32::MAX;
        let mut next_coords = (0, 0);
        for (i, row) in record_grid.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                for p in item.paths.values() {
                    if !p.extended {
                        if p.distance < min_tentative_dist {
                            min_tentative_dist = p.distance;
                            next_coords = (i, j);
                        }
                    }
                }
            }
        }
        (row, col) = next_coords;
        if !record_grid.iter().any(|row| row.iter().any(|item| item.paths.values().any(|x| !x.extended))) {
            break;
        }

    }
    let best_path = record_grid[dest.0][dest.1].paths.values().min_by_key(|&x| x.distance).unwrap();
    dbg!(best_path);
    best_path.distance
}

#[derive(Clone, Debug)]
struct TraversalRecord {
    paths: HashMap<(Direction, u32), Path>,
}

#[derive(Clone, Debug)]
struct Path {
    distance: u32,
    path: Vec<Direction>,
    extended: bool,
}

impl Path {
    fn num_repeats(&self) -> (Direction, u32) {
        if let Some(last) = self.path.last() {
            let mut repeats = 0;
            for d in self.path.iter().rev() {
                if d == last {repeats += 1;}
                else { break; }
            }
            return (*last, repeats);
        }
        (Direction::Up, 0)
    }

}

impl Default for TraversalRecord {
    fn default() -> Self {
        Self {
            paths: HashMap::new(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}