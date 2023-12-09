use crate::utils::{FileIter, Solves};
use std::cmp::max;
use std::collections::HashMap;
use std::iter::{zip, Map};

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 2;
    type ParsedInput = Map<FileIter, fn(String) -> (u32, Vec<BallSet>)>;
    type Output = u32;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        Self::read_file(dir).map(parse_line)
    }

    fn part1(dir: &str) -> Self::Output {
        let max_values = [12u32, 13u32, 14u32];
        let mut total = 0;
        for (id, handful) in Self::parse_input(dir) {
            let mut valid = true;
            for ball_set in handful {
                if zip(ball_set, max_values).any(|(a, b)| a > b) {
                    valid = false;
                    break;
                }
            }
            if valid {
                total += id;
            }
        }
        total
    }
    fn part2(dir: &str) -> Self::Output {
        let mut total = 0;
        for (_, handful) in Self::parse_input(dir) {
            total += get_power(get_max_seen(handful))
        }
        total
    }
}

type BallSet = [u32; 3]; // RGB
                         // todo make this a struct

fn parse_line(line: String) -> (u32, Vec<BallSet>) {
    let (game, handfuls_str) = line.split_once(": ").unwrap();
    let (_, id_str) = game.split_once(" ").unwrap();
    let id = id_str.parse::<u32>().unwrap();

    let handfuls = handfuls_str.split("; ");
    let ball_sets = handfuls.map(parse_handful).collect();
    return (id, ball_sets);
}

fn parse_handful(handful: &str) -> BallSet {
    let ball_counts = handful.split(", ");
    let mut hashmap = HashMap::from([("red", 0u32), ("green", 0u32), ("blue", 0u32)]);
    for count in ball_counts {
        let (number, colour) = count.split_once(" ").unwrap();
        let new_total = *hashmap.get_mut(colour).unwrap() + number.parse::<u32>().unwrap();
        hashmap.insert(colour, new_total);
    }
    let red = *hashmap.get("red").unwrap();
    let green = *hashmap.get("green").unwrap();
    let blue = *hashmap.get("blue").unwrap();
    return [red, green, blue];
}

fn get_power(ball_set: BallSet) -> u32 {
    ball_set.iter().product()
}

fn get_max_seen(handful: Vec<BallSet>) -> BallSet {
    let mut max_r = 0u32;
    let mut max_g = 0u32;
    let mut max_b = 0u32;

    for [red, green, blue] in handful {
        max_r = max(red, max_r);
        max_g = max(green, max_g);
        max_b = max(blue, max_b);
    }
    [max_r, max_g, max_b]
}
