mod utils;
use utils::Solves;
mod day01;
mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
// mod day07;

fn main() {
    let dir = "input";
    day01::Solution::print_solutions(dir);
    // for s in [day01::Solution] {
    //     s::print_solutions(dir);
    // }
}
