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
        let unfolded_input = input.into_iter().map(|(row, numbers)| unfold(row, numbers, 5)).collect();
        count_total_arrangements(unfolded_input)
    }
}
fn parse_line(line: String) -> (String, Vec<usize>) {
    let (map, numbers_str) = line.split_once(" ").unwrap();
    let numbers: Vec<usize> = numbers_str.split(",").map(|x| x.parse().unwrap()).collect();
    (map.to_string(), numbers)
}

fn count_total_arrangements(input: Vec<(String, Vec<usize>)>) -> usize {
    let mut total = 0;
    let steps = input.len();
    let mut i = 1;
    for (row, numbers) in input {
        let ways = count_arrangements(&row[0..], &numbers[0..]);
        println!("done step {}/{}", i, steps);
        i += 1;
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
    if row.chars().filter(|x| *x != OPERATIONAL).count() < numbers.iter().sum() {
        return 0;
    }
    if numbers.len() == 1 {
        if n > row.len() {return 0;}
        if n == row.len() {return 1;}
        let (mindex, maxdex) = find_min_max_indices(n, row);
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
        let fits = block_fits(n, i, row);
        if fits {
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

fn unfold(row: String, numbers: Vec<usize>, unfold_factor: usize) -> (String, Vec<usize>) {
    let unfolded_row = vec![row;unfold_factor].join(UNKNOWN.to_string().as_str());
    let mut unfolded_numbers = Vec::new();
    for _ in 0..unfold_factor {
        let mut n = numbers.clone();
        unfolded_numbers.append(&mut n);
    }
    (unfolded_row, unfolded_numbers)
}

fn is_valid(row: String, numbers: &Vec<usize>) -> bool {
    let white = row.replace(".", " ");
    let sections: Vec<usize> = white.split_whitespace().map(|x| x.len()).collect();
    sections == *numbers
}

fn brute_force_line(row: String, numbers: Vec<usize>) -> usize {
    let mut total = 0;
    let num_unk = row.chars().filter(|x| *x == '?').count();
    for mut i in 0..2usize.pow(num_unk as u32) {
        let indices = row.match_indices("?");
        let mut attempt: Vec<_> = row.chars().collect();
        for (index, _) in indices {
            let c = if (i&1) == 0 {'.'} else {'#'};
            attempt[index] = c;
            i = i>>1;
        }

        if is_valid(attempt.into_iter().collect(), &numbers) {
            total += 1;
        }
    }
    total

}