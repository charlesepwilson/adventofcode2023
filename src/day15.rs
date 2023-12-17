use crate::utils::Solves;

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 15;
    type ParsedInput = Vec<String>;
    type Output = u32;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        let mut line_iter = Self::read_file(dir);
        let line = line_iter.next().unwrap();
        let steps = line.split(",").map(|x| x.to_string());
        steps.collect()
    }

    fn part1(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        dbg!(holiday_ascii_string_helper("HASH"));
        let mut total: u32 = 0;
        for step in input {
            total += holiday_ascii_string_helper(step.as_str()) as u32;
        }
        total
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        0
    }
}

fn holiday_ascii_string_helper(s: &str) -> u8 {
    let mut current_value: u8 = 0;
    for c in s.chars() {
        let ascii_code = c as u8;
        current_value = current_value.wrapping_add(ascii_code);
        current_value = current_value.wrapping_mul(17);
    }
    current_value
}
