use regex::Regex;
use std::collections::HashMap;
use crate::utils::{FileIter, Solves};

const DIGITS: [(&str, u32); 9] = [
    ("1", 1u32),
    ("2", 2u32),
    ("3", 3u32),
    ("4", 4u32),
    ("5", 5u32),
    ("6", 6u32),
    ("7", 7u32),
    ("8", 8u32),
    ("9", 9u32),
];

const NUMBERS: [(&str, u32); 9] = [
    ("one", 1u32),
    ("two", 2u32),
    ("three", 3u32),
    ("four", 4u32),
    ("five", 5u32),
    ("six", 6u32),
    ("seven", 7u32),
    ("eight", 8u32),
    ("nine", 9u32),
];

pub struct Solution;
impl Solves for Solution {
    const DAY: u32 = 1;
    type T = FileIter;

    fn parse_input(dir: &str) -> Self::T {
        Self::read_file(dir)
    }

    fn part1(dir: &str) -> u32 {
        let hashmap = HashMap::from(DIGITS);
        sum_calibration_digits(hashmap, Self::parse_input(dir))
    }

    fn part2(dir: &str) -> u32 {
        let mut hashmap = HashMap::from(DIGITS);
        hashmap.extend(NUMBERS);
        sum_calibration_digits(hashmap, Self::parse_input(dir))
    }
}


fn sum_calibration_digits(valid_strings: HashMap<&str, u32>, lines: FileIter) -> u32 {
    let regex_parts: Vec<&str> = valid_strings.clone().into_keys().collect();
    let re = Regex::new(regex_parts.join("|").as_str()).unwrap();
    let mut reverse_regex_parts: Vec<String> = Vec::new();
    for p in regex_parts {
        let rev_p = p.chars().rev().collect::<String>();
        reverse_regex_parts.push(rev_p);
    }
    let rev_re = Regex::new(reverse_regex_parts.join("|").as_str()).unwrap();
    let mut sum = 0u32;
    for line in lines {
        sum += combine_first_last(line, valid_strings.clone(), &re, &rev_re);
    }
    sum
}

fn combine_first_last(
    line: String,
    valid_strings: HashMap<&str, u32>,
    re: &Regex,
    rev_re: &Regex,
) -> u32 {
    let first = re.find(line.as_str()).unwrap().as_str();
    let reverse_line = line.chars().rev().collect::<String>();
    let rev_last = rev_re.find(reverse_line.as_str()).unwrap().as_str();
    let last = rev_last.chars().rev().collect::<String>();
    let first_digit = valid_strings.get(first).unwrap();
    let last_digit = valid_strings.get(last.as_str()).unwrap();
    first_digit * 10 + last_digit
}
