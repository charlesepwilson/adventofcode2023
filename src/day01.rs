use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1() {
    let mut sum = 0u32;
    if let Ok(lines) = read_lines("./input/day01.txt") {
        for line in lines {
            if let Ok(ip) = line {
                sum += combine_first_last(ip);
            }
        }
    }
    println!("Day 01, Part 1: sum is {}", sum)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_first_digit<I>(line_iter: I) -> u32
where I: Iterator<Item = char>
{
    for character in line_iter {
        if character.is_ascii_digit() {
            return character.to_digit(10u32).unwrap();
        }
    }
    panic!("Invalid input");
}

fn combine_first_last(line: String) -> u32 {
    let first = find_first_digit(line.chars());
    let last = find_first_digit(line.chars().rev());
    first * 10 + last
}
