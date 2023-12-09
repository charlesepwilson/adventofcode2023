use std::collections::HashMap;
use crate::utils::read_lines;

pub fn part1() -> u32 {
    let (instructions, map) = parse_input();
    follow_instructions(instructions, map)
}
pub fn part2() -> u64 {
    let (instructions, map) = parse_input();
    let start_nodes = get_start_nodes(&map);
    let valid_targets_groups = get_valid_targets(start_nodes, instructions, map);
    let mut valid_target_indices = Vec::new();
    for g in valid_targets_groups {
        valid_target_indices.push(g[0]);
    }
    valid_target_indices.iter().map(|&x| x as u64).reduce(lcm).unwrap()
}

fn parse_input() -> (Vec<Instruction>, NodeMap) {
    let mut hm = HashMap::new();
    if let Ok(mut buf_lines) = read_lines("./input/day08.txt") {
        let l1 = buf_lines.next().unwrap().unwrap();
        let lr = parse_lr_line(l1);
        buf_lines.next();
        for line in buf_lines {
            if let Ok(ip) = line {
                let (k, (l, r)) = parse_map_line(ip);
                hm.insert(k, (l, r));
            }
        }
        return (lr, hm);
    }
    panic!("File handling issue")
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
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

fn parse_lr(ch: char) -> Instruction {if ch == 'L' { Instruction::Left} else { Instruction::Right}}
fn parse_lr_line(line: String) -> Vec<Instruction> {
    line.chars().map(parse_lr).collect()
}

fn parse_map_line(line: String) -> (Node, (Node, Node))
{
    let (key, values_str) = line.split_once(" = ").unwrap();
    let (l, r) = values_str.strip_prefix("(").unwrap().strip_suffix(")").unwrap().split_once(", ").unwrap();
    (alias_str(key), (alias_str(l), alias_str(r)))
}


fn follow_instructions(instructions: Vec<Instruction>, map: NodeMap) -> u32
{
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
type Node = [u16;3];
type NodeMap = HashMap<Node, (Node, Node)>;
fn alias_str(s: &str) -> Node {
    // aaargh lifetimes are such a pain...converting to a fixed size owned type :/
    let mut a = [0u16;3];
    for (i, x) in s.encode_utf16().enumerate() {
        if i < 3 {a[i] = x}
    }
    a
}

fn get_valid_targets(start_nodes: Vec<Node>, instructions: Vec<Instruction>, map: NodeMap) ->Vec<Vec<usize>> {
    // condenses a map of node -> (node, node) with an instruction list to choose the path
    // to a single map from (index, node) -> node where index is the nth instruction
    // keep track of the overall index for each starting node so that we can work out at what point each
    // starting node gets back to somewhere it's already been, and hence is in a loop, so that we can use maths to work out the smallest intersection point
    let mut indexed_map: HashMap<(usize, Node), (usize, Node)> = HashMap::new();
    let mut loop_info_vec = Vec::new();
    for start_node in start_nodes {
        let mut node = start_node.clone();
        let mut valid_targets = Vec::new();
        for (overall_index, (i, instruction)) in instructions.iter().enumerate().cycle().enumerate() {
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

fn is_start_node(node: &Node) -> bool {node[2] == 65}
fn is_end_node(node: &Node) -> bool {node[2] == 90}


fn get_start_nodes(node_map: &NodeMap) -> Vec<Node> {
    node_map.keys().map(|x| *x).filter(is_start_node).collect()
}


fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {a} else {gcd(b, a % b)}
}
