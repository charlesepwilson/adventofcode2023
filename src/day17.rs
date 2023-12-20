use std::cmp::{min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
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
        // let input = Self::parse_input(dir);
        // find_optimal_path::<Node1>(input)
        0
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        find_optimal_path::<Node2>(input)
    }
}

fn find_optimal_path<N>(input: Vec<Vec<u32>>) -> u32
where N: Node
{
    // for each grid location, there are actually 12 nodes (once we apply the 3-in-a-row constraint)
    // that is, [(x, y, Up(1)), (x, y, Up(2)), ...(x, y, Right(3))
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    let mut sorted_distances = BinaryHeap::new();
    let start_node = N::default();
    let target_nodes = N::construct_possible_nodes(input.len()-1, input[0].len()-1, &input);
    distances.insert(start_node, 0);
    let mut current_node = start_node;
    loop {
        let neighbours = current_node.find_neighbours(&input);
        let unvisited_neighbours = neighbours.difference(&visited);
        for n in unvisited_neighbours {
            let heat_loss = n.get_heat_loss(&input);
            let new_distance = distances.get(&current_node).unwrap() + heat_loss;
            if let Some(d) = distances.get_mut(n) {
                let min_d = min(*d, new_distance);
                *d = min_d;
                // this is the point at which the sorted distances can obtain visited nodes, since we can't easily remove the old distance when we add a new one
                // might need to consider using a different structure than a binary heap :/
                sorted_distances.push(Reverse((min_d, *n)));
            }
            else {
                distances.insert(*n, new_distance);
                sorted_distances.push(Reverse((new_distance, *n)));
            }
        }
        visited.insert(current_node);
        while visited.contains(&current_node) {
            current_node = sorted_distances.pop().unwrap().0.1;
        }
        if visited.is_superset(&target_nodes) {
            break;
        }
    }
    let possible_distances = target_nodes.iter().map(|x| distances.get(x).unwrap());
    *possible_distances.min().unwrap()
}



trait Node: Default + Clone + Copy + Eq + PartialEq + Ord + PartialOrd + Hash + Debug {
    const MIN_STEPS: u32;
    const MAX_STEPS: u32;
    fn get_row(&self) -> usize;
    fn get_col(&self) -> usize;
    fn get_entry_path(&self) -> EntryPath;
    fn traverse_edge(&self, direction: Direction, grid: &Vec<Vec<u32>>) -> Option<Self> {
        let row = self.get_row();
        let col = self.get_col();
        let entry_path = self.get_entry_path();
        if direction == entry_path.direction.opposite() {return None;}
        let (col_change, row_change) = direction.coordinate_change();
        let new_row_o = row.checked_add_signed(row_change);
        if new_row_o.is_none() {return None;}
        let new_col_o = col.checked_add_signed(col_change);
        if new_col_o.is_none() {return None;}
        let (new_row, new_col) = (new_row_o.unwrap(), new_col_o.unwrap());
        if (new_row >= grid.len()) || (new_col >= grid[0].len()) {
            return None;
        }
        let new_steps;
        if direction == entry_path.direction {
            if entry_path.steps >= Self::MAX_STEPS {
                return None;
            }
            else {
                new_steps = entry_path.steps + 1;
            }
        }
        else {
            if entry_path.steps < Self::MIN_STEPS {
                return None;
            }
            new_steps = 1;
        }

        let new_entry_path = EntryPath {
            direction,
            steps: new_steps,
        };
        let new_node = Self::from_values(new_row, new_col, new_entry_path);
        Some(new_node)
    }

    fn from_values(row: usize, col: usize, entry_path: EntryPath) -> Self;

    fn find_neighbours(&self, grid: &Vec<Vec<u32>>) -> HashSet<Self> {
        let mut neighbours = HashSet::new();
        for direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            if let Some(neighbour) = self.traverse_edge(direction, grid) {
                neighbours.insert(neighbour);
            }
        }
        neighbours
    }
    fn get_heat_loss(&self, grid: &Vec<Vec<u32>>) -> u32 {
        grid[self.get_row()][self.get_col()]
    }

    fn construct_possible_nodes(row: usize, col: usize, grid: &Vec<Vec<u32>>) -> HashSet<Self> {
        let mut nodes = HashSet::new();
        let rows = grid.len();
        let cols = grid[0].len();

        let max_up = rows - row - 1;
        let max_down = row;
        let max_right = cols;
        let max_left = cols - col - 1;

        for (direction, max_steps) in [
            (Direction::Up, max_up),
            (Direction::Down, max_down),
            (Direction::Right, max_right),
            (Direction::Left, max_left),
        ] {
            let actual_max = min(Self::MAX_STEPS, max_steps as u32);
            for steps in Self::MIN_STEPS..=actual_max {
                nodes.insert(Self::from_values(row, col, EntryPath {direction, steps}));
            }
        }
        nodes
    }


}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy, Ord, PartialOrd, Default)]
struct Node1 {
    row: usize,
    col: usize,
    entry_path: EntryPath,
}


impl Node for Node1 {
    const MIN_STEPS: u32 = 1;
    const MAX_STEPS: u32 = 3;

    fn get_row(&self) -> usize {
        self.row
    }

    fn get_col(&self) -> usize {
        self.col
    }

    fn get_entry_path(&self) -> EntryPath {
        self.entry_path
    }

    fn from_values(row: usize, col: usize, entry_path: EntryPath) -> Self {
        Self {row, col, entry_path}
    }
}


#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy, Ord, PartialOrd, Default)]
struct Node2 {
    row: usize,
    col: usize,
    entry_path: EntryPath,
}

impl Node for Node2 {
    const MIN_STEPS: u32 = 4;
    const MAX_STEPS: u32 = 10;

    fn get_row(&self) -> usize {
        self.row
    }

    fn get_col(&self) -> usize {
        self.col
    }

    fn get_entry_path(&self) -> EntryPath {
        self.entry_path
    }

    fn from_values(row: usize, col: usize, entry_path: EntryPath) -> Self {
        Self {row, col, entry_path}
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
struct EntryPath {
    direction: Direction,
    steps: u32,  // number of repeated steps
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
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

impl Default for Direction {
    fn default() -> Self {
        Self::Down
    }
}