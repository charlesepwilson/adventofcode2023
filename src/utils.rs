use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::iter::FilterMap;
use std::ops::{Div, Mul, Rem};
use std::path::Path;

pub trait Solves {
    const DAY: u32;
    fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(BufReader::new(file).lines())
    }
    fn get_file_path(dir: &str) -> String {
        format!("./{}/day{:02}.txt", dir, Self::DAY)
    }
    fn read_file(dir: &str) -> FileIter {
        let Ok(lines) = Self::read_lines(Self::get_file_path(dir)) else {
            todo!()
        };
        lines.filter_map(|s| s.ok())
    }
    type ParsedInput;
    type Output: Display;
    fn parse_input(dir: &str) -> Self::ParsedInput;
    fn part1(dir: &str) -> Self::Output;
    fn part2(dir: &str) -> Self::Output;
    fn _print(part: u32, value: Self::Output) {
        println!("Day {} Part {}: {}", Self::DAY, part, value);
    }
    fn print_part1(dir: &str) {
        Self::_print(1, Self::part1(dir));
    }
    fn print_part2(dir: &str) {
        Self::_print(2, Self::part2(dir));
    }
    fn print_solutions(dir: &str) {
        Self::print_part1(dir);
        Self::print_part2(dir);
    }
}

pub type FileIter = FilterMap<Lines<BufReader<File>>, fn(io::Result<String>) -> Option<String>>;

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Div<Output = T> + Mul<Output = T> + Copy + Eq + Default + Rem<Output = T>,
{
    a * (b / gcd(a, b))
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Div<Output = T> + Mul<Output = T> + Copy + Eq + Default + Rem<Output = T>,
{
    if b == T::default() {
        a
    } else {
        gcd(b, a % b)
    }
}
