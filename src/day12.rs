use std::cmp::min;
use crate::utils::Solves;

pub struct Solution;
const OPERATIONAL: char = '.';
const DAMAGED: char = '#';
const UNKNOWN: char = '?';

impl Solves for Solution {
    const DAY: u32 = 12;
    type ParsedInput = Vec<(String, Vec<usize>)>;
    type Output = usize;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        Self::read_file(dir).map(parse_line).collect()
    }

    fn part1(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        count_total_arrangements(input)
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        let unfolded = input;
        // count_total_arrangements(unfolded)
        0
    }
}
fn parse_line(line: String) -> (String, Vec<usize>) {
    let (map, numbers_str) = line.split_once(" ").unwrap();
    let numbers: Vec<usize> = numbers_str.split(",").map(|x| x.parse().unwrap()).collect();
    (map.to_string(), numbers)
}

fn count_total_arrangements(input: Vec<(String, Vec<usize>)>) -> usize {
    let mut total = 0;
    for (row, numbers) in input {
        let ways = count_arrangements(&row[0..], &numbers[0..]);
        dbg!(ways);
        total += ways;
    }
    total
}


fn find_min_max_indices(block_size: usize, row: &str) -> (usize, usize) {
    // assumes all #s in row form one continuous block
    let matches = row.match_indices(DAMAGED);
    let mut mindex = 0;
    let mut maxdex = row.len().saturating_sub(block_size);
    if let Some(min_hash) = matches.clone().map(|x| x.0).min() {
        maxdex = min(min_hash, row.len() - block_size);
    }
    if let Some(max_hash) = matches.map(|x| x.0).max() {
        mindex = max_hash.saturating_sub(block_size - 1);
    };
    (mindex, maxdex)
}

fn count_arrangements(row: &str, numbers: &[usize]) -> usize {
    let n = numbers[0];
    let mut total = 0;

    if numbers.len() == 1 {
        if n > row.len() {return 0;}
        if n == row.len() {return 1;}
        let (mindex, maxdex) = find_min_max_indices(n, row);
        // dbg!(row, n, mindex, maxdex);
        for i in mindex..=maxdex {
            if !row[i..i+n].contains(OPERATIONAL) {
                total += 1;
            }
        }
        return total;
    }

    let wiggle_room = row.len() + 1 - numbers.iter().sum::<usize>() - numbers.len();
    let max = min(row.find(DAMAGED).unwrap_or(wiggle_room), wiggle_room);
    for i in 0..=max {
        if block_fits(n, i, row) {
            let ways = count_arrangements(&row[(i + n + 1)..], &numbers[1..]);
            total += ways;
        }
    }
    total
}

fn block_fits(block_size: usize, at_index: usize, row: &str) -> bool {
    let padded_row = ".".to_string() + row + ".";
    let enough_space = !padded_row[(at_index+1)..(at_index+block_size+1)].contains(OPERATIONAL);
    let left_ok = padded_row.chars().nth(at_index).unwrap() != DAMAGED;
    let right_ok = padded_row.chars().nth(at_index+block_size+1).unwrap() != DAMAGED;
    enough_space && left_ok && right_ok
}