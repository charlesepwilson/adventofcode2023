use std::cmp::max;
use std::collections::HashMap;
use std::iter::zip;
use crate::utils::read_lines;

pub fn part1() -> u32 {
    let max_values = [12u32, 13u32, 14u32];
    let mut total = 0u32;
    for (id, handful) in parse_input() {
        let mut valid = true;
        for ball_set in handful {
            if zip(ball_set, max_values).any(|(a, b)| a > b) {
                valid = false;
                break
            }
        }
        if valid {total += id;}
    }
    total
}

pub fn part2() -> u32 {
    let mut total = 0u32;
    for (_, handful) in parse_input() {
        total += get_power(get_max_seen(handful))
    }
    total
}
type BallSet = [u32;3];  // RGB

fn parse_input() -> Vec<(u32, Vec<BallSet>)> {
    let mut result: Vec<(u32, Vec<BallSet>)> = Vec::new();
    if let Ok(lines) = read_lines("./input/day02.txt") {
        for line in lines {
            if let Ok(ip) = line {
                result.push(parse_line(ip));
            }
        }
    }
    result
}

fn parse_line(line: String) -> (u32, Vec<BallSet>) {
    let (game, handfuls_str) = line.split_once(": ").unwrap();
    let (_, id_str) = game.split_once(" ").unwrap();
    let id = id_str.parse::<u32>().unwrap();

    let handfuls = handfuls_str.split("; ");
    let ball_sets = handfuls.map(parse_handful).collect();
    return (id, ball_sets)
}

fn parse_handful(handful: &str) -> BallSet {
    let ball_counts = handful.split(", ");
    let mut hashmap = HashMap::from(
        [
            ("red", 0u32),
            ("green", 0u32),
            ("blue", 0u32),
        ]
    );
    for count in ball_counts {
        let (number, colour) = count.split_once(" ").unwrap();
        let new_total = *hashmap.get_mut(colour).unwrap() + number.parse::<u32>().unwrap();
        hashmap.insert(colour, new_total);
    }
    let red = *hashmap.get("red").unwrap();
    let green = *hashmap.get("green").unwrap();
    let blue = *hashmap.get("blue").unwrap();
    return [red, green, blue]
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