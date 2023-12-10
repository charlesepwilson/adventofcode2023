mod utils;
use utils::Solves;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

macro_rules! solve {
    (
        $dir:literal,
        $(
            $x:ident
        ),+$(,)?
    ) => {
        $( $x::Solution::print_solutions($dir);)+
    }
}

fn main() {
    solve!("input", day01, day02, day03, day04, day05, day06, day07, day08, day09, day10);
}
