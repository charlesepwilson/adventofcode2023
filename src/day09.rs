use crate::utils;
use crate::utils::Solves;
use std::iter::zip;
use std::ops::{Add, Div, Mul};

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 9;
    type ParsedInput = Vec<Vec<i128>>;
    type Output = i128;
    fn parse_input(dir: &str) -> Self::ParsedInput {
        let mut sequences = Vec::new();
        for line in Self::read_file(dir) {
            sequences.push(
                line.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            );
        }
        sequences
    }
    fn part1(dir: &str) -> Self::Output {
        let sequences = Self::parse_input(dir);
        sequences.into_iter().map(get_next_value).sum()
    }

    fn part2(dir: &str) -> Self::Output {
        let sequences = Self::parse_input(dir);
        sequences.into_iter().map(get_previous_value).sum()
    }
}

fn compute_value_at_index(sequence: Vec<i128>, index: i128) -> i128 {
    let polynomial = get_polynomial(sequence);
    polynomial.compute_value(index)
}
fn get_next_value(sequence: Vec<i128>) -> i128 {
    let len = sequence.len() as i128;
    compute_value_at_index(sequence, len)
}
fn get_previous_value(sequence: Vec<i128>) -> i128 {
    compute_value_at_index(sequence, -1)
}

fn get_polynomial(sequence: Vec<i128>) -> Polynomial {
    // y(i) = sequence[i]
    let size = sequence.len();
    let mut p = Polynomial::zeros(size - 1);
    for (j, y_j) in sequence.iter().enumerate() {
        p = p + ((*y_j) * p_j(j, size));
    }
    p
}

fn p_j(j: usize, sequence_len: usize) -> Polynomial {
    // Lagrange polynomials
    let mut numerator = Polynomial::reduced(vec![1], 1);
    let jay = j as i128;
    let mut denominator = 1;
    for k in 0..(sequence_len as i128) {
        if k == jay {
            continue;
        }
        denominator *= jay - k;
        let expression = Polynomial::reduced(vec![-k, 1], 1);
        numerator = expression * numerator;
    }
    numerator / (denominator)
}

struct Polynomial {
    coefficients: Vec<i128>,
    divisor: i128,
}

impl Polynomial {
    fn reduced(coefficients: Vec<i128>, divisor: i128) -> Self {
        let mut p = Self {
            coefficients,
            divisor,
        };
        p.reduce();
        p
    }
    fn reduce(&mut self) {
        let mut gcd = self.divisor;
        for c in self.coefficients.iter() {
            gcd = utils::gcd(gcd, *c);
        }
        self.coefficients = self.coefficients.iter().map(|x| x / gcd).collect();
        self.divisor = self.divisor / gcd;
    }
    fn compute_value(&self, value: i128) -> i128 {
        let t: i128 = self
            .coefficients
            .iter()
            .enumerate()
            .map(|(power, c)| c * value.pow(power as u32))
            .sum();
        t / self.divisor
    }

    fn zeros(degree: usize) -> Self {
        Self {
            coefficients: vec![0; degree + 1],
            divisor: 1,
        }
    }

    fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let divisor = utils::lcm(self.divisor, rhs.divisor);
        let left_mul = divisor / self.divisor;
        let right_mul = divisor / rhs.divisor;
        let new_left = self.coefficients.iter().map(|x| x * left_mul);
        let new_right = rhs.coefficients.iter().map(|x| x * right_mul);
        let coefficients = zip(new_left, new_right).map(|(l, r)| l + r).collect();

        Polynomial::reduced(coefficients, divisor)
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self::zeros(self.degree() + rhs.degree());
        for (s, left) in self.coefficients.iter().enumerate() {
            for (o, right) in rhs.coefficients.iter().enumerate() {
                let exponent = s + o;
                let value = left * right;
                result.coefficients[exponent] += value;
            }
        }
        result.reduce();
        result
    }
}

impl Div<i128> for Polynomial {
    type Output = Self;

    fn div(self, rhs: i128) -> Self::Output {
        Polynomial::reduced(self.coefficients, self.divisor * rhs)
    }
}

impl Mul<Polynomial> for i128 {
    type Output = Polynomial;

    fn mul(self, rhs: Polynomial) -> Self::Output {
        let mut coefficients: Vec<i128> = rhs.coefficients.iter().map(|x| x * self).collect();
        let mut gcd = rhs.divisor;
        for c in coefficients.iter() {
            gcd = utils::gcd(gcd, *c);
        }
        let divisor = rhs.divisor / gcd;
        coefficients = coefficients.iter().map(|x| x / gcd).collect();

        Polynomial::reduced(coefficients, divisor)
    }
}
