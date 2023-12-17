use crate::utils::Solves;

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 14;
    type ParsedInput = Vec<Vec<u8>>;  // by column rather than row
    type Output = usize;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        let mut result = Vec::new();
        for line in Self::read_file(dir) {
            for (i, &c) in line.as_bytes().iter().enumerate() {
                if result.len() <= i {
                    result.push(Vec::new());
                }
                result[i].push(c);
            }
        }
        result
    }

    fn part1(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        let mut total = 0;
        for col in input {
            total += calculate_load(col);
        }
        total
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        0
    }
}

const ROUND: u8 = b'O';
const FIXED: u8 = b'#';
const BLANK: u8 = b'.';


fn calculate_load(column: Vec<u8>) -> usize {
    let len = column.len() as isize;
    let mut total_load = 0;
    let mut last_fixed = -1;
    for (i, &item) in column.iter().enumerate() {
        if item == FIXED {
            last_fixed = i as isize;
        }
        else if item == ROUND {
            last_fixed += 1;
            total_load += len - last_fixed;
        }
    }
    total_load as usize
}

