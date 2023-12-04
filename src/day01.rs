use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

pub fn part1() -> u32 {
    let hashmap = HashMap::from(DIGITS);
    sum_calibration_digits(hashmap)
}

pub fn part2() -> u32 {
    let mut hashmap = HashMap::from(DIGITS);
    hashmap.extend(NUMBERS);
    sum_calibration_digits(hashmap)
}

fn sum_calibration_digits(valid_strings: HashMap<&str, u32>) -> u32 {
    let regex_parts: Vec<&str> = valid_strings.clone().into_keys().collect();
    let re = Regex::new(regex_parts.join("|").as_str()).unwrap();
    let mut reverse_regex_parts: Vec<String> = Vec::new();
    for p in regex_parts {
        let rev_p = p.chars().rev().collect::<String>();
        reverse_regex_parts.push(rev_p);
    }
    let rev_re = Regex::new(reverse_regex_parts.join("|").as_str()).unwrap();
    let mut sum = 0u32;
    if let Ok(lines) = read_lines("./input/day01.txt") {
        for line in lines {
            if let Ok(ip) = line {
                sum += combine_first_last(ip, valid_strings.clone(), &re, &rev_re);
            }
        }
    }
    sum
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
