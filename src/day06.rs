use std::iter::{zip, Zip};
use std::vec::IntoIter;
use crate::utils::read_lines;

pub fn part1() -> usize {
    let mut ways = Vec::new();
    let input = parse_input1();
    for (time, record) in input {
        let (t1, t2) = get_button_times_from_dist(record, time);
        ways.push(integers_between(t1, t2));
    }
    ways.iter().product()
}
pub fn part2() -> usize {
    let (time, record) = parse_input2();
    let (t1, t2) = get_button_times_from_dist(record, time);
    integers_between(t1, t2)
}

fn parse_file() -> Vec<String> {
    let mut lines = Vec::new();
    if let Ok(buf_lines) = read_lines("./input/day06.txt") {
        for line in buf_lines {
            if let Ok(ip) = line {
                lines.push(ip);
            }
        }
    }
    lines
}

fn parse_input1() -> Zip<IntoIter<usize>, IntoIter<usize>> {
    let lines = parse_file();
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

fn parse_input2() -> (usize, usize) {
    let lines = parse_file();
    let (_, times_str) = lines[0].split_once(" ").unwrap();
    let (_, records_str) = lines[1].split_once(" ").unwrap();
    let time = times_str.replace(" ", "").parse().unwrap();
    let record = records_str.replace(" ", "").parse().unwrap();
    (time, record)
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
