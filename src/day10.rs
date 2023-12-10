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
        loop_size / 2
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        let mut junkless: Vec<Vec<char>> = Vec::new();
        for d in [Direction::Left, Direction::Up, Direction::Right] {
            if let Some(j) = remove_junk(&input, d) {
                junkless = j;
            }
        }
        let mut num_enclosed: usize = 0;
        for line in junkless {
            let mut s: String = line.iter().collect();
            s = s.replace("-", "");
            s = s.replace("LJ", "");
            s = s.replace("F7", "");
            s = s.replace("L7", "|");
            s = s.replace("FJ", "|");
            let mut inside: bool = false;
            for c in s.chars() {
                if inside & is_enclosable(c) {
                    num_enclosed += 1;
                }
                if c == '|' {
                    inside = !inside;
                }
            }
        }
        num_enclosed
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
fn is_enclosable(c: char) -> bool {
    c == '.'
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
        let new_row_o = input.get(current_row);
        if new_row_o == None {
            return None;
        }
        let new_row = new_row_o.unwrap();

        let new_pipe = new_row[current_column];
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

fn remove_junk(input: &Vec<Vec<char>>, start_direction: Direction) -> Option<Vec<Vec<char>>> {
    let num_rows = input.len();
    let num_columns = input[0].len();
    let mut new_input = vec![vec!['.'; num_columns]; num_rows];
    let (mut current_row, mut current_column) = find_start(&input);
    let mut current_direction = start_direction;

    loop {
        let (row_change, column_change) = current_direction.index_change();
        current_row = add_signed(current_row, row_change);
        current_column = add_signed(current_column, column_change);
        let row_o = input.get(current_row);
        if row_o == None {
            return None;
        }
        let row = row_o.unwrap();
        let new_pipe = row[current_column];
        new_input[current_row][current_column] = new_pipe;
        if is_start(new_pipe) {
            new_input[current_row][current_column] = generate_pipe(current_direction, start_direction);
            return Some(new_input);
        }
        if let Some(next_dir) = follow_pipe(current_direction, new_pipe) {
            current_direction = next_dir;
        } else {
            return None;
        }
    }
}


fn generate_pipe(from: Direction, to: Direction) -> char {
    match (from, to) {
        (Direction::Right, Direction::Right) => '-',
        (Direction::Right, Direction::Down) => '7',
        (Direction::Right, Direction::Up) => 'J',
        (Direction::Down, Direction::Right) => 'L',
        (Direction::Down, Direction::Down) => '|',
        (Direction::Down, Direction::Left) => 'J',
        (Direction::Left, Direction::Down) => 'F',
        (Direction::Left, Direction::Left) => '-',
        (Direction::Left, Direction::Up) => 'L',
        (Direction::Up, Direction::Right) => 'F',
        (Direction::Up, Direction::Left) => '7',
        (Direction::Up, Direction::Up) => '|',
        (_, _) => panic!(),
    }
}