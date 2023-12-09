use crate::utils::Solves;
use std::iter::{zip, Zip};
use std::vec::IntoIter;

pub struct Solution;
impl Solves for Solution {
    const DAY: u32 = 6;
    type ParsedInput = Vec<String>;
    type Output = usize;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        Self::read_file(dir).collect()
    }
    fn part1(dir: &str) -> Self::Output {
        let mut ways = Vec::new();
        let input = Self::parse_input1(dir);
        for (time, record) in input {
            let (t1, t2) = get_button_times_from_dist(record, time);
            ways.push(integers_between(t1, t2));
        }
        ways.iter().product()
    }

    fn part2(dir: &str) -> Self::Output {
        let (time, record) = Self::parse_input2(dir);
        let (t1, t2) = get_button_times_from_dist(record, time);
        integers_between(t1, t2)
    }
}

impl Solution {
    fn parse_input1(dir: &str) -> Zip<IntoIter<usize>, IntoIter<usize>> {
        let lines = Self::parse_input(dir);
        let mut times: Vec<usize> = Vec::new();
        let mut records: Vec<usize> = Vec::new();
        let times_str = lines[0].split_whitespace();
        let records_str = lines[1].split_whitespace();
        for t in times_str {
            if let Ok(v) = t.parse::<usize>() {
                times.push(v);
            }
        }
        for r in records_str {
            if let Ok(v) = r.parse::<usize>() {
                records.push(v);
            }
        }
        zip(times, records)
    }

    fn parse_input2(dir: &str) -> (usize, usize) {
        let lines = Self::parse_input(dir);
        let (_, times_str) = lines[0].split_once(" ").unwrap();
        let (_, records_str) = lines[1].split_once(" ").unwrap();
        let time = times_str.replace(" ", "").parse().unwrap();
        let record = records_str.replace(" ", "").parse().unwrap();
        (time, record)
    }
}

fn get_button_times_from_dist(dist: usize, total_time: usize) -> (f64, f64) {
    // the graph of distance against button time is a parabola centred on (total_time/2)
    // hitting the axis at 0 and total_time
    // we work out the 2 possible button times for the record using the quadratic
    // then all button times between them will be better
    let discrim = total_time.pow(2u32) - 4 * dist;
    let offset = f64::sqrt(discrim as f64) / 2f64;
    let midpoint = (total_time as f64) / 2f64;
    (midpoint - offset, midpoint + offset)
}

fn integers_between(a: f64, b: f64) -> usize {
    (b.ceil() as usize) - (a.floor() as usize) - 1
}
