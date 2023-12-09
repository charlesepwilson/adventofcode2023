use crate::utils::read_lines;

pub fn part1() -> i64 {
    let sequences = parse_input();
    sequences.iter().map(get_next_value).sum()
}
pub fn part2() -> i64 {
    0
}

fn parse_input() -> Vec<Vec<i64>> {
    let mut sequences = Vec::new();
    if let Ok(buf_lines) = read_lines("./input/day09.txt") {
        for line in buf_lines {
            if let Ok(ip) = line {
                sequences.push(
                    ip.split_whitespace().map(|x| x.parse().unwrap()).collect()
                );
            }
        }
    }
    sequences
}

fn get_next_line(sequence: &Vec<i64>) -> Vec<i64> {
    let mut next_line = Vec::new();
    for i in 0..(sequence.len() - 1) {
        next_line.push(sequence[i+1] - sequence[i]);
    }
    next_line
}

fn get_next_value(sequence: &Vec<i64>) -> i64 {
    let last = *sequence.last().unwrap();
    let second_last = *sequence.get(sequence.len() - 2).unwrap();
    let diff = last - second_last;
    if diff == 0 {
        return last;
    }
    return last + get_next_value(&get_next_line(sequence));

}
