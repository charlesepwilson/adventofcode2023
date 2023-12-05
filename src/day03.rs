use std::cmp::max;
use std::collections::VecDeque;
use regex::{Match, Regex};
use crate::utils::read_lines;

pub fn part1() -> usize {
    let input = get_vec_of_lines();
    let re = Regex::new(r"\d+").unwrap();
    let part_numbers = collect_part_numbers(&input, re);
    part_numbers.iter().sum()
}

pub fn part2() -> usize {
    let mut r = 0;
    let input = get_vec_of_lines();
    for (row, line) in input.iter().enumerate() {
        for (column, _) in line.match_indices("*") {
            let ratio = get_gear_ratio(row, column, &input);
            r += ratio.unwrap_or(0);
        }
    }
    r
}

fn get_gear_ratio(line_number: usize, col_number: usize, input: &Vec<String>) -> Option<usize> {
    let mut part_numbers = Vec::new();
    let mut seen_indices: Vec<(usize, usize)> = Vec::new();
    for i in 0..=2 {
        let row_number = line_number + i - 1;
        if let Some(line) = input.get(row_number){
            for j in 0..=2 {
                let index = col_number + j - 1;
                if seen_indices.contains(&(row_number, index)) {continue;}
                if !line.chars().nth(index).unwrap().is_ascii_digit() {continue;}
                let (part_num, seen) = find_full_number(index, line);
                part_numbers.push(part_num);
                for index in seen {seen_indices.push((row_number, index));}
            }
        }
    }
    if part_numbers.len() != 2 { return None;}
    Some(part_numbers.iter().product())
}

fn find_full_number(index: usize, string: &String) -> (usize, Vec<usize>) {
    let mut chars = VecDeque::new();
    let mut seen_indices = Vec::new();
    let (left, right) = string.split_at(index);
    for (i, c) in left.chars().rev().enumerate() {
        seen_indices.push(index - i);
        if c.is_ascii_digit() {
            chars.push_front(c)
        }
        else {
            break
        }
    }
    for (i, c) in right.chars().enumerate() {
        seen_indices.push(index + i);
        if c.is_ascii_digit() {
            chars.push_back(c)
        }
        else {
            break
        }
    }
    (chars.iter().collect::<String>().parse::<usize>().unwrap(), seen_indices)
}

fn get_vec_of_lines() -> Vec<String>{
    let mut result: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./input/day03.txt") {
        for line in lines {
            if let Ok(ip) = line {
                result.push(ip);
            }
        }
    }
    result
}


fn is_part(line_number: usize, mat: Match, input: &Vec<String>) -> Option<usize> {
    let ln = line_number as isize;
    let it = (ln-1)..=(ln+1);
    let mp = it.map(|x| input.get(x as usize));
    for l in mp {
        if l != None {
            let line = l.unwrap();
            let start = max(0, mat.start() as isize - 1) as usize;
            if (start..=mat.end()).map(
                    |x| line.chars().nth(x)
                ).any(
                    |c|is_symbol(c.unwrap_or('.'))
                ) {
                return mat.as_str().parse().ok();
            }
        }
    }
    None
}


fn is_symbol(c: char) -> bool {
    !(c.is_ascii_alphanumeric() || c == '.')
}

fn collect_part_numbers(input: &Vec<String>, re: Regex) -> Vec<usize> {
    let mut r = Vec::new();
    for (i, line) in input.iter().enumerate() {
        for m in re.find_iter(line.as_str()) {
            r.push(is_part(i, m, input).unwrap_or(0));
        }
    }
    r
}