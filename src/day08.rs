use std::collections::HashMap;
use crate::utils::read_lines;

pub fn part1() -> u32 {
    let (instructions, map) = parse_input();
    follow_instructions(instructions, map)
}
pub fn part2() -> u32 {
    // let input = parse_input();
    0
}

fn parse_input() -> (Vec<Instruction>, HashMap<Node, (Node, Node)>) {
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
        println!("{:?}", hm);
        return (lr, hm);
    }
    panic!("File handling issue")
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Left,
    Right,
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

fn follow_instructions(instructions: Vec<Instruction>, map: HashMap<Node, (Node, Node)>) -> u32
{
    let target = alias_str("ZZZ");
    println!("{:?}", target);
    let mut node = alias_str("AAA");
    for (i, instruction) in instructions.iter().cycle().enumerate() {
        let (l, r) = *map.get(&node).unwrap();
        node = match instruction {
            Instruction::Left => l, Instruction::Right => r,
        };
        if node == target {
            return (i + 1) as u32;
        }
    }
    panic!("Somehow iterated past infinity")
}
type Node = [u16;3];
fn alias_str(s: &str) -> Node {
    // aaargh lifetimes are such a pain...converting to a fixed size owned type :/
    let mut a = [0u16;3];
    for (i, x) in s.encode_utf16().enumerate() {
        if i < 3 {a[i] = x}
    }
    a
}