use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Div, Mul, Rem};
use std::path::Path;


pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn lcm<T>(a: T, b: T) -> T
where T: Div<Output=T>, T: Mul<Output=T>, T: Copy, T: Eq, T: Default, T: Rem<Output=T> {
    a * (b / gcd(a, b))
}

pub fn gcd<T>(a: T, b: T) -> T
where T: Div<Output=T>, T: Mul<Output=T>, T: Copy, T: Eq, T: Default, T: Rem<Output=T> {
    if b == T::default() {a} else {gcd(b, a % b)}
}