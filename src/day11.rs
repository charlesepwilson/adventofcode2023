use crate::utils::Solves;
use std::cmp::{max, min};

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 11;
    type ParsedInput = Vec<Vec<char>>;
    type Output = usize;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        let result = Self::read_file(dir).map(|x| x.chars().collect()).collect();
        result
    }

    fn part1(dir: &str) -> Self::Output {
        let input = Solution::parse_input(dir);
        sum_distances(input, 2)
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Solution::parse_input(dir);
        sum_distances(input, 1_000_000)
    }
}

fn sum_distances(input: Vec<Vec<char>>, expansion_value: usize) -> usize {
    let empty_rows = record_empty_rows(&input);
    let empty_cols = record_empty_rows(&transpose(&input));
    let galaxies = record_galaxy_positions(&input);
    let mut total_dist = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for j in 0..i {
            let other_galaxy = galaxies[j];
            let dist = get_distance(
                *galaxy,
                other_galaxy,
                &empty_rows,
                &empty_cols,
                expansion_value,
            );
            total_dist += dist;
        }
    }
    total_dist
}

fn record_empty_rows(rows: &Vec<Vec<char>>) -> Vec<usize> {
    let mut empty_rows = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        if !row.contains(&'#') {
            empty_rows.push(i);
        }
    }
    empty_rows
}

fn transpose<T>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Default + Clone + Copy,
{
    let num_rows = matrix.len();
    let num_cols = matrix[0].len();
    let mut result = vec![vec![T::default(); num_rows]; num_cols];
    for (i, r) in matrix.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            result[j][i] = *c;
        }
    }
    result
}

fn record_galaxy_positions(input: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies.push((i, j));
            }
        }
    }
    galaxies
}

fn get_expanded_dist(a: usize, b: usize, empty_rows: &Vec<usize>, expansion_value: usize) -> usize {
    let high = max(a, b);
    let low = min(a, b);
    let num_expansions = empty_rows
        .iter()
        .filter(|&&x| (x > low) & (x < high))
        .count();
    high - low + (num_expansions * (expansion_value - 1))
}

fn get_distance(
    a: (usize, usize),
    b: (usize, usize),
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
    expansion_value: usize,
) -> usize {
    let (a_row, a_col) = a;
    let (b_row, b_col) = b;
    let row_dist = get_expanded_dist(a_row, b_row, empty_rows, expansion_value);
    let col_dist = get_expanded_dist(a_col, b_col, empty_cols, expansion_value);
    row_dist + col_dist
}
