use std::collections::HashMap;
use crate::utils::read_lines;


pub fn part1() -> usize {
    let mut r = 0;
    for (_, winning, own) in parse_input() {
        r += compute_score(winning, own);
    }
    r
}
pub fn part2() -> usize {
    let mut r = 0;
    let mut gen_map = HashMap::new();
    let input = parse_input();
    let (max_card, _, _) = input.clone().into_iter().last().unwrap();
    for (card, winning, own) in input.into_iter().rev() {
        let extras = list_extras(card, winning, own, max_card);
        let mut cards_added = 1;
        for extra in extras {
            cards_added += gen_map.get(&extra).unwrap();
        }
        gen_map.insert(card, cards_added);
        r += cards_added;
    }
    r
}


fn parse_input() -> Vec<(usize, Vec<usize>, Vec<usize>)>{
    let mut result = Vec::new();
    if let Ok(lines) = read_lines("./input/day04.txt") {
        for line in lines {
            if let Ok(ip) = line {
                result.push(parse_line(ip));
            }
        }
    }
    result
}

fn parse_line(line: String) -> (usize, Vec<usize>, Vec<usize>) {
    let (card, nums) = line.split_once(": ").unwrap();
    let card_num = card.split_whitespace().last().unwrap().parse().unwrap();
    let (winning, own) = nums.split_once(" | ").unwrap();
    let winning_numbers = parse_numbers(winning);
    let own_numbers = parse_numbers(own);
    (card_num, winning_numbers, own_numbers)
}
fn parse_numbers(string: &str) -> Vec<usize> {
    string.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn compute_wins(winning_numbers: Vec<usize>, own_numbers: Vec<usize>) -> usize {
    let mut wins = 0;
    for n in own_numbers {
        if winning_numbers.contains(&n) {
            wins += 1;
        }
    }
    wins
}

fn compute_score(winning_numbers: Vec<usize>, own_numbers: Vec<usize>) -> usize {
    let wins = compute_wins(winning_numbers, own_numbers);
    if wins < 1 {return 0;}
    2usize.pow((wins - 1) as u32)
}

fn list_extras(card: usize, winning_numbers: Vec<usize>, own_numbers: Vec<usize>, max_card: usize) -> Vec<usize> {
    let wins = compute_wins(winning_numbers, own_numbers);
    (card+1..=card+wins).filter(|&x| x <= max_card).collect()
}
