use crate::utils::{lcm, Solves};
use std::collections::HashMap;

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 8;
    type ParsedInput = (Vec<Instruction>, NodeMap);
    type Output = u64;
    fn parse_input(dir: &str) -> Self::ParsedInput {
        let mut hm = HashMap::new();
        let mut file_iter = Self::read_file(dir);
        let l1 = file_iter.next().unwrap();
        let lr = parse_lr_line(l1);
        file_iter.next();
        for line in file_iter {
            let (k, (l, r)) = parse_map_line(line);
            hm.insert(k, (l, r));
        }
        return (lr, hm);
    }
    fn part1(dir: &str) -> Self::Output {
        let (instructions, map) = Self::parse_input(dir);
        follow_instructions(instructions, map) as u64
    }

    fn part2(dir: &str) -> Self::Output {
        let (instructions, map) = Self::parse_input(dir);
        let start_nodes = get_start_nodes(&map);
        let valid_targets_groups = get_valid_targets(start_nodes, instructions, map);
        let mut valid_target_indices = Vec::new();
        for g in valid_targets_groups {
            valid_target_indices.push(g[0]);
        }
        valid_target_indices
            .iter()
            .map(|&x| x as u64)
            .reduce(lcm)
            .unwrap()
    }
}
#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn follow<T>(&self, options: (T, T)) -> T {
        match self {
            Self::Left => options.0,
            Self::Right => options.1,
        }
    }
}

fn parse_lr(ch: char) -> Instruction {
    if ch == 'L' {
        Instruction::Left
    } else {
        Instruction::Right
    }
}
fn parse_lr_line(line: String) -> Vec<Instruction> {
    line.chars().map(parse_lr).collect()
}

fn parse_map_line(line: String) -> (Node, (Node, Node)) {
    let (key, values_str) = line.split_once(" = ").unwrap();
    let (l, r) = values_str
        .strip_prefix("(")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .split_once(", ")
        .unwrap();
    (alias_str(key), (alias_str(l), alias_str(r)))
}

fn follow_instructions(instructions: Vec<Instruction>, map: NodeMap) -> u32 {
    let target = alias_str("ZZZ");
    let mut node = alias_str("AAA");
    for (i, instruction) in instructions.iter().cycle().enumerate() {
        let options = *map.get(&node).unwrap();
        node = instruction.follow(options);
        if node == target {
            return (i + 1) as u32;
        }
    }
    panic!("Somehow iterated past infinity")
}
type Node = [u16; 3];
type NodeMap = HashMap<Node, (Node, Node)>;
fn alias_str(s: &str) -> Node {
    // aaargh lifetimes are such a pain...converting to a fixed size owned type :/
    let mut a = [0u16; 3];
    for (i, x) in s.encode_utf16().enumerate() {
        if i < 3 {
            a[i] = x
        }
    }
    a
}

fn get_valid_targets(
    start_nodes: Vec<Node>,
    instructions: Vec<Instruction>,
    map: NodeMap,
) -> Vec<Vec<usize>> {
    // condenses a map of node -> (node, node) with an instruction list to choose the path
    // to a single map from (index, node) -> node where index is the nth instruction
    // keep track of the overall index for each starting node so that we can work out at what point each
    // starting node gets back to somewhere it's already been, and hence is in a loop, so that we can use maths to work out the smallest intersection point
    let mut indexed_map: HashMap<(usize, Node), (usize, Node)> = HashMap::new();
    let mut loop_info_vec = Vec::new();
    for start_node in start_nodes {
        let mut node = start_node.clone();
        let mut valid_targets = Vec::new();
        for (overall_index, (i, instruction)) in instructions.iter().enumerate().cycle().enumerate()
        {
            if is_end_node(&node) {
                valid_targets.push(overall_index);
            }
            let options = *map.get(&node).unwrap();
            let next_node = instruction.follow(options);
            if indexed_map.contains_key(&(i, node)) {
                loop_info_vec.push(valid_targets);
                break;
            }
            indexed_map.insert((i, node), (overall_index, next_node));
            node = next_node;
        }
    }
    loop_info_vec
}

fn is_start_node(node: &Node) -> bool {
    node[2] == 65
}
fn is_end_node(node: &Node) -> bool {
    node[2] == 90
}

fn get_start_nodes(node_map: &NodeMap) -> Vec<Node> {
    node_map.keys().map(|x| *x).filter(is_start_node).collect()
}


// For a given start point a0, if we find an endpoint at z0 and determine that the path has a loop size of L0,
// then the equation for the step numbers when you hit that endpoint again is:
// z = z0 + n*L0, where n is any integer
//
// If you have another start point a1, then that has the same equation for its own values:
// z = z1 + m*L1, where m is also any integer
//
// To find the the step number where both of them hit an endpoint,
// we just have to set these equal and then find the minimum n and m:
// z0 + n*L0 = z1 + m*L1
// n*L0 - m*L1 = (z1 - z0)
// This is equivalent to the equation described in the Extended Euclidean Algorithm
// with a = L0, b = L1, x = n, y = m, and c = (z1 - z0)
// where we choose z1 > z0 for convenience
// note that for valid solutions to exist, we must have (z1 - z0) be a multiple of gcd(L0, L1)
// and that we may need to divide the entire equation through by gcd(L0, L1) to get the
// exact equation described in the algorithm
//
// once we have found the appropriate n and m, we can then substitute back into
// z = z0 + n*L0
// to find the step number where they both hit an endpoint
//
// This then gives us a new loop, where this z is our new z0,
// and the loop size is L0_new = lcm(L0, L1)
// we can then repeat this process with the new loop as loop 0
// and the loop from another start point as loop 1
// until we have included all of our endpoints
//
// the only problem with this solution is that it assumes each loop has only one endpoint
// the simple way to resolve this is to just repeat this process for each combination of endpoints
// i.e. if we had 3 start points (a, b, c), each with 2 endpoints in their loop, we would need to solve for
// (a0, b0, c0), (a0, b0, c1), (a0, b1, c0), ... , (a1, b1, c1)
// and then choose the lowest valid solution found
// idk if there's a more effective way to do this, but this wouldn't be relevant for the actual puzzle input
// but would definitely solve it for the generic case