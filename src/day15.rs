use crate::utils::Solves;
use std::collections::HashMap;

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

        let mut total: u32 = 0;
        for step in input {
            total += holiday_ascii_string_helper(step.as_str()) as u32;
        }
        total
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        compute_focus_power(input)
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

fn compute_focus_power(steps: Vec<String>) -> u32 {
    let mut lenses: HashMap<&str, (usize, usize)> = HashMap::new();
    for (mut i, step) in steps.iter().enumerate() {
        let (label, focal_length_str) = step.split_once(|x| (x == '-') || (x == '=')).unwrap();
        if focal_length_str == "" {
            lenses.remove(label);
        } else {
            let focal_length: usize = focal_length_str.parse().unwrap();
            if let Some((old_i, _)) = lenses.get(label) {
                i = *old_i;
            }
            lenses.insert(label, (i, focal_length));
        }
    }

    let mut boxes = HashMap::new();

    for (&label, &(insertion_step, _)) in lenses.iter() {
        let box_num = holiday_ascii_string_helper(label) as usize;
        if !boxes.contains_key(&box_num) {
            boxes.insert(box_num, Vec::new());
        }
        let bx = boxes.get_mut(&box_num).unwrap();
        if insertion_step >= bx.len() {
            bx.resize(insertion_step + 1, "");
        }
        bx[insertion_step] = label;
    }

    let mut total_focus_power = 0;
    for (box_num, bx) in boxes {
        for (i, label) in bx.into_iter().filter(|&x| x != "").enumerate() {
            let slot_num = i + 1;
            total_focus_power += (box_num + 1) * slot_num * lenses.get(label).unwrap().1;
        }
    }
    total_focus_power as u32
}
