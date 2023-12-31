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
mod day11;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;

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
    // solve!("examples", day20);
    solve!("input", day20);

    // solve!(
    //     "input", day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11,
    //     day13, day14, day15, day16, day17, day18, day19
    // );
}
