use std::collections::VecDeque;
use std::iter::zip;
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
        let mut total = 0;
        for (m, n) in input {
            total += compute_arrangements(m, n);
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

fn condense_row(map: Vec<char>) -> Vec<(char, usize)> {
    // Converts the row into pairs of (char, count)
    let mut map_deque: VecDeque<_> = map.into_iter().collect();
    let mut map_chain = Vec::new();
    let mut counter: usize = 0;
    let mut previous_char = OPERATIONAL;
    map_deque.push_back(OPERATIONAL);
    while !map_deque.is_empty() {
        let c = map_deque.pop_front().unwrap();
        counter += 1;
        if c != previous_char {
            map_chain.push((previous_char, counter));
            counter = 0;
            previous_char = c;
        }
    }
    map_chain
}

fn compute_arrangements(map: String, numbers: Vec<usize>) -> usize {
    let better_map = fill_in_hashes(map, &numbers);
    let even_better_map = fill_in_blanks(better_map, &numbers);
    let map_chain = condense_row(even_better_map);
    0
}

fn compute_unknown_arrangements2(num_unknown: usize, first_group: usize, second_group: usize) -> usize {
    // if you just have a group of ?s, with 2 groups of #s inside,
    // this is the number of ways of arranging them
    let n = num_unknown - first_group - second_group;
    (n * (n+1)) / 2
}

fn compute_unknown_arrangements3(
    num_unknown: usize,
    first_group: usize,
    second_group: usize,
    third_group: usize,
) -> usize {
    // if you just have a group of ?s, with 3 groups of #s inside,
    // this is the number of ways of arranging them
    let n = num_unknown - first_group - second_group - third_group;
    (n * (n+1) * (n-1)) / 6
}

fn compute_unknown_arrangements(
    num_unknown: usize,
    numbers: &Vec<usize>,
) -> usize {
    let l = numbers.len();
    if l == 2 {
        compute_unknown_arrangements2(num_unknown, numbers[0], numbers[1])
    }
    else if l == 3 {
        compute_unknown_arrangements3(num_unknown, numbers[0], numbers[1], numbers[2])
    }
    else {
        let mut total = 0;
        for i in 1..=(num_unknown - numbers.iter().sum::<usize>() - (l - 1)) {
            total += compute_unknown_arrangements(
                num_unknown - numbers[0] - i,
                &(numbers[1..].iter().map(|x| *x).collect()),
            );
        }
        total
    }
}


pub fn guarantee_hashes(size: usize, numbers: &Vec<usize>) -> Vec<char> {
    let total_required_space: usize = numbers.iter().sum::<usize>() + numbers.len() - 1;
    let v: usize = size - total_required_space;
    let mut row = vec![OPERATIONAL;size];
    for (index, n) in numbers.iter().enumerate() {
        let num_hashes = n.saturating_sub(v);
        let previous_nums = &numbers[0..index];
        let offset = previous_nums.iter().sum::<usize>() + previous_nums.iter().count();
        for i in (n-num_hashes)..*n {
            row[i + offset] = DAMAGED;
        }

    }
    row
}

pub fn fill_in_hashes(row: String, numbers: &Vec<usize>) -> Vec<char> {
    let total_size = row.chars().count();
    let guaranteed_hashes = guarantee_hashes(total_size, &numbers);
    let combined = zip(row.chars(), guaranteed_hashes).map(|(r, g)| if g == DAMAGED {g} else {r}).collect();
    combined
}

fn fill_in_blanks(row: Vec<char>, numbers: &Vec<usize>) -> Vec<char> {
    let s: String = row.iter().collect();
    for n in numbers {
        let pat: String = (0..*n).map(|_| DAMAGED).collect();
        let found = s.find(pat.as_str()).unwrap();

    }
    row
}
