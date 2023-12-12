use crate::utils::Solves;

pub struct Solution;
const OPERATIONAL: char = '.';
const DAMAGED: char = '#';
const UNKNOWN: char = '?';

impl Solves for Solution {
    const DAY: u32 = 12;
    type ParsedInput = Vec<(String, Vec<usize>)>;
    type Output = u32;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        Self::read_file(dir).map(parse_line).collect()
    }

    fn part1(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        let mut total = 0;
        for (row, numbers) in input {
            total += iterate_possibilities(row, numbers);
        }
        total
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        0
    }
}

fn parse_line(line: String) -> (String, Vec<usize>) {
    let (map, numbers_str) = line.split_once(" ").unwrap();
    let numbers: Vec<usize> = numbers_str.split(",").map(|x| x.parse().unwrap()).collect();
    (map.to_string(), numbers)
}

fn is_valid(mut row: String, numbers: &Vec<usize>) -> bool {
    // dbg!(&row);
    loop {
        let l = row.chars().count();
        row = row.replace("..", ".");
        if row.chars().count() == l {break;}
    }
    row = row.strip_prefix(".").unwrap_or(row.as_str()).to_string();
    row = row.strip_suffix(".").unwrap_or(row.as_str()).to_string();
    let sections = row.split(".");
    let damage_runs: Vec<_> = sections.map(|x| x.chars().count()).collect();
    // dbg!(&row, numbers, &damage_runs);
    &damage_runs == numbers
}

fn iterate_possibilities(row: String, numbers: Vec<usize>) -> u32 {
    let mut num_valid = 0u32;
    let unknown_indices: Vec<_> = row.match_indices(UNKNOWN).collect();
    let num_unknowns = unknown_indices.len() as u32;

    let mut r: Vec<_> = row.chars().collect();
    for x in 0..(2u32.pow(num_unknowns)) {
        for (i, (n, _)) in unknown_indices.iter().enumerate() {
            // dbg!(x, n, i);
            let bit = (x >> i) & 1;
            let c = if bit == 0 {DAMAGED} else {OPERATIONAL};
            r[*n] = c;
        }
        let s: String = r.iter().collect();
        let valid = is_valid(s, &numbers);
        if valid { num_valid += 1;}
    }
    num_valid
}