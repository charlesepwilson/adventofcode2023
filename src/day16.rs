use crate::utils::Solves;
use std::cmp::max;

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 16;
    type ParsedInput = Vec<Vec<u8>>;
    type Output = u32;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        Self::read_file(dir)
            .map(|x| x.as_bytes().iter().map(|y| *y).collect())
            .collect()
    }

    fn part1(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        compute_total_activated(0, 0, Direction::Right, &input)
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        try_all_start_points(input)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const _BLANK: u8 = b'.';
const H_SPLITTER: u8 = b'-';
const V_SPLITTER: u8 = b'|';
const R_MIRROR: u8 = b'/';
const L_MIRROR: u8 = b'\\';

impl Direction {
    fn traverse_panel(&self, panel: u8) -> Vec<Self> {
        match (self, panel) {
            (Self::Up, R_MIRROR) => vec![Self::Right],
            (Self::Up, L_MIRROR) => vec![Self::Left],
            (Self::Up, H_SPLITTER) => vec![Self::Right, Self::Left],

            (Self::Down, R_MIRROR) => vec![Self::Left],
            (Self::Down, L_MIRROR) => vec![Self::Right],
            (Self::Down, H_SPLITTER) => vec![Self::Right, Self::Left],

            (Self::Left, R_MIRROR) => vec![Self::Down],
            (Self::Left, L_MIRROR) => vec![Self::Up],
            (Self::Left, V_SPLITTER) => vec![Self::Up, Self::Down],

            (Self::Right, R_MIRROR) => vec![Self::Up],
            (Self::Right, L_MIRROR) => vec![Self::Down],
            (Self::Right, V_SPLITTER) => vec![Self::Up, Self::Down],

            (&d, _) => vec![d],
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

fn traverse_grid(
    start_row: usize,
    start_col: usize,
    start_direction: Direction,
    grid: &Vec<Vec<u8>>,
) -> Vec<Vec<TraversalRecord>> {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut record_grid = vec![vec![TraversalRecord::default(); num_cols]; num_rows];
    send_path(
        start_row,
        start_col,
        start_direction,
        &grid,
        &mut record_grid,
    );
    record_grid
}

fn send_path(
    row: usize,
    col: usize,
    direction: Direction,
    grid: &Vec<Vec<u8>>,
    record_grid: &mut Vec<Vec<TraversalRecord>>,
) {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let panel = grid[row][col];
    if record_grid[row][col].entered_from.contains(&direction) {
        // been here before
        return;
    }

    record_grid[row][col].activated = true;
    record_grid[row][col].entered_from.push(direction);

    let new_directions = direction.traverse_panel(panel);

    for d in new_directions {
        let (col_change, row_change) = d.coordinate_change();
        let (row_o, col_o) = (
            row.checked_add_signed(row_change),
            col.checked_add_signed(col_change),
        );
        if row_o.is_none()
            || col_o.is_none()
            || row_o.unwrap() >= num_rows
            || col_o.unwrap() >= num_cols
        {
            // Gone outside grid
            continue;
        }
        let (new_row, new_col) = (row_o.unwrap(), col_o.unwrap());
        send_path(new_row, new_col, d, grid, record_grid);
    }
}

#[derive(Clone, Debug)]
struct TraversalRecord {
    activated: bool,
    entered_from: Vec<Direction>,
}

impl Default for TraversalRecord {
    fn default() -> Self {
        Self {
            activated: false,
            entered_from: Vec::new(),
        }
    }
}

fn compute_total_activated(
    start_row: usize,
    start_col: usize,
    start_direction: Direction,
    grid: &Vec<Vec<u8>>,
) -> u32 {
    let record_grid = traverse_grid(start_row, start_col, start_direction, grid);
    let mut total: u32 = 0;
    for row in record_grid {
        for item in row {
            if item.activated {
                total += 1;
            }
        }
    }
    total
}

fn try_all_start_points(grid: Vec<Vec<u8>>) -> u32 {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut max_value = 0;
    for i in 0..num_rows {
        max_value = max(
            max_value,
            compute_total_activated(i, 0, Direction::Right, &grid),
        );
    }
    for i in 0..num_rows {
        max_value = max(
            max_value,
            compute_total_activated(i, num_cols - 1, Direction::Left, &grid),
        );
    }
    for i in 0..num_cols {
        max_value = max(
            max_value,
            compute_total_activated(0, i, Direction::Down, &grid),
        );
    }
    for i in 0..num_rows {
        max_value = max(
            max_value,
            compute_total_activated(num_rows - 1, i, Direction::Up, &grid),
        );
    }
    max_value
}
