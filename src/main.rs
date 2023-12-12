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
mod day12;
mod day12bruteforce;

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
    // dbg!(day12::fill_in_hashes("???.###".to_string(), &vec![1, 1, 3]));
    // dbg!(day12::fill_in_hashes(".??..??...?##.".to_string(), &vec![1, 1, 3]));
    // dbg!(day12::fill_in_hashes("?#?#?#?#?#?#?#?".to_string(), &vec![1, 3, 1, 6]));
    // dbg!(day12::fill_in_hashes("????.#...#...".to_string(), &vec![4,1,1]));
    // dbg!(day12::fill_in_hashes("????.######..#####.".to_string(), &vec![1,6,5]));
    // dbg!(day12::fill_in_hashes("?###????????".to_string(), &vec![3,2,1]));

    solve!("input", day12bruteforce);
    // solve!("input", day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11);
}
