use std::cmp::max;
use regex::{Match, Regex};
use crate::utils::read_lines;

pub fn part1() -> usize {
    let input = get_vec_of_lines();
    let re = Regex::new(r"\d+").unwrap();
    let part_numbers = collect_part_numbers(&input, re);
    part_numbers.iter().sum()
}

type CharMatrix = Vec<String>;

fn get_vec_of_lines() -> CharMatrix{
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


fn is_part(line_number: usize, mat: Match, input: &CharMatrix) -> Option<usize> {
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

fn collect_part_numbers(input: &CharMatrix, re: Regex) -> Vec<usize> {
    let mut r = Vec::new();
    for (i, line) in input.iter().enumerate() {
        for m in re.find_iter(line.as_str()) {
            r.push(is_part(i, m, input).unwrap_or(0));
        }
    }
    r
}