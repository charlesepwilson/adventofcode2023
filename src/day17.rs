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
        find_optimal_path((num_rows - 1, num_cols - 1), (0, 0), input)
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
    record_grid[row][col].distance = 0;

    loop {
        for d in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            let (col_change, row_change) = d.coordinate_change();
            if let (Some(new_row), Some(new_col)) = (row.checked_add_signed(row_change), col.checked_add_signed(col_change)) {
                if (new_row < num_rows) && (new_col < num_cols) {
                    if !record_grid[new_row][new_col].visited {
                        record_grid[new_row][new_col].distance = record_grid[row][col].distance + grid[new_row][new_col];
                    }
                }
            }
        }
        record_grid[row][col].visited = true;
        if record_grid.iter().all(|x| x.iter().all(|y| y.visited)) { break; }
        let mut min_tentative_dist = u32::MAX;
        let mut min_coords = (0, 0);
        for (i, row) in record_grid.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if !item.visited {
                    if item.distance < min_tentative_dist {
                        min_tentative_dist = item.distance;
                        min_coords = (i, j);
                    }
                }

            }
        }
        (row, col) = min_coords;
    }
    dbg!(&record_grid);

    let mut total = 0;
    let mut last_direction = Direction::Down;
    let mut num_repeated_dirs = 0;
    let (mut r, mut c) = dest;
    while (r, c) != start {
        let node = grid[r][c];
        let mut min_v = u32::MAX;
        let mut min_dir = Direction::Down;
        total += node;
        for d in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            if (d == last_direction) && (num_repeated_dirs >= 2) {continue;}
            if d == last_direction.opposite() {continue;}
            let (col_change, row_change) = d.coordinate_change();
            if let (Some(new_row), Some(new_col)) = (r.checked_add_signed(row_change), c.checked_add_signed(col_change)) {
                if (new_row < num_rows) && (new_col < num_cols) {
                    let value = record_grid[new_row][new_col].distance;
                    if value < min_v {
                        min_v = value;
                        min_dir = d;
                    }
                }
            }
        }
        if min_dir == last_direction {
            num_repeated_dirs += 1;
        }
        else {
            num_repeated_dirs = 0;
            last_direction = min_dir;
        }
        let (col_change, row_change) = min_dir.coordinate_change();
        r = r.checked_add_signed(row_change).unwrap();
        c = c.checked_add_signed(col_change).unwrap();
    }
    total
}

#[derive(Clone, Debug)]
struct TraversalRecord {
    visited: bool,
    distance: u32,
}

impl Default for TraversalRecord {
    fn default() -> Self {
        Self {
            visited: false,
            distance: u32::MAX,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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