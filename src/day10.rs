use crate::utils::Solves;

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 10;
    type ParsedInput = Vec<Vec<char>>;
    type Output = usize;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        Self::read_file(dir).map(|x| x.chars().collect()).collect()
    }

    fn part1(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        let start = find_start(&input);
        let mut loop_size = 0;
        for d in [Direction::Left, Direction::Up, Direction::Right] {
            if let Some(l) = find_loop_size(start, &input, d) {
                loop_size = l;
            }
        }
        loop_size/2
    }

    fn part2(dir: &str) -> Self::Output {
        dir.chars().into_iter().count()
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn index_change(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

fn follow_pipe(start_direction: Direction, pipe_char: char) -> Option<Direction> {
    match (start_direction, pipe_char) {
        (Direction::Up, '|') => Some(Direction::Up),
        (Direction::Up, 'F') => Some(Direction::Right),
        (Direction::Up, '7') => Some(Direction::Left),
        (Direction::Down, '|') => Some(Direction::Down),
        (Direction::Down, 'J') => Some(Direction::Left),
        (Direction::Down, 'L') => Some(Direction::Right),
        (Direction::Left, '-') => Some(Direction::Left),
        (Direction::Left, 'F') => Some(Direction::Down),
        (Direction::Left, 'L') => Some(Direction::Up),
        (Direction::Right, '-') => Some(Direction::Right),
        (Direction::Right, '7') => Some(Direction::Down),
        (Direction::Right, 'J') => Some(Direction::Up),
        (_, _) => None,
    }
}

fn is_start(c: char) -> bool {
    c == 'S'
}

fn find_start(input: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if is_start(*c) {
                return (i, j);
            }
        }
    }
    panic!("Couldn't find start point")
}

fn add_signed(a: usize, b: isize) -> usize {
    ((a as isize) + b) as usize
}

fn find_loop_size(
    start_point: (usize, usize),
    input: &Vec<Vec<char>>,
    start_direction: Direction,
) -> Option<usize> {
    let (mut current_row, mut current_column) = start_point;
    let mut current_direction = start_direction;
    let mut steps_taken = 0;
    loop {
        let (row_change, column_change) = current_direction.index_change();
        current_row = add_signed(current_row, row_change);
        current_column = add_signed(current_column, column_change);
        steps_taken += 1;
        let new_pipe = input[current_row][current_column];
        if is_start(new_pipe) {
            return Some(steps_taken);
        }
        if let Some(next_dir) = follow_pipe(current_direction, new_pipe) {
            current_direction = next_dir;
        } else {
            return None;
        }
    }
}
