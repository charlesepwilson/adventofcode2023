use std::collections::HashSet;
use crate::utils::read_lines;

pub fn part1() -> usize {
    let mut input = parse_input();
    input.sort_by_cached_key(sort_key);
    let winnings = input.iter().enumerate().map(|(rank, (_, bid))| bid * (rank + 1));
    winnings.sum()
}
pub fn part2() -> usize {
    let mut input = parse_input();
    input.sort_by_cached_key(joker_sort_key);
    let winnings = input.iter().enumerate().map(|(rank, (_, bid))| bid * (rank + 1));
    winnings.sum()
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
enum Card {
    Joker = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            // Rust won't let me just parse the characters to digits and make enum members from those :/
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
enum HandType {
    Empty,
    HighCard,
    Pair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl HandType {
    fn upgrade(self) -> Self {
        match self {
            Self::Empty => Self::HighCard,
            Self::HighCard => Self::Pair,
            Self::Pair => Self::ThreeOfKind,
            Self::TwoPair => Self::FullHouse,
            Self::ThreeOfKind => Self::FourOfKind,
            _ => Self::FiveOfKind,
        }
    }
}


type Hand = Vec<Card>;


fn parse_input() -> Vec<(Hand, usize)> {
    let mut lines = Vec::new();
    if let Ok(buf_lines) = read_lines("./input/day07.txt") {
        for line in buf_lines {
            if let Ok(ip) = line {
                let (cards_str, bid_str) = ip.split_once(" ").unwrap();
                let hand = cards_str.chars().map(Card::from).collect();
                lines.push((hand, bid_str.parse().unwrap()));
            }
        }
    }
    lines
}

fn categorise_hand(hand: &Hand) -> HandType {
    let (distinct, count) = get_distinct_and_count(hand);
    categorise_from_distinct_count(distinct, count)
}
fn get_distinct_and_count(hand: &Hand) -> (usize, usize) {
    let card_set: HashSet<_> = hand.iter().map(|x| *x).collect();
    let distinct = card_set.len();
    let mut counts = Vec::new();
    for card in hand {
        counts.push(hand.iter().filter(|&x| x == card).count());
    }
    let max_count = counts.iter().max().unwrap_or(&0);
    (distinct, *max_count)
}

fn categorise_from_distinct_count(distinct: usize, count: usize) -> HandType {
    match (distinct, count) {
        (_, 0) => HandType::Empty,
        (_, 1) => HandType::HighCard,
        (4, 2) => HandType::Pair,
        (_, 2) => HandType::TwoPair,
        (2, 3) => HandType::FullHouse,
        (3, 3) => HandType::ThreeOfKind,
        (_, 4) => HandType::FourOfKind,
        (_, 5) => HandType::FiveOfKind,
        _ => panic!("{} {}", distinct, count),
    }
}

fn sort_key(h: &(Hand, usize)) -> (HandType, Hand) {
    let (hand, _) = h;
    (categorise_hand(hand), hand.iter().map(|x| *x).collect())
    // This is bad...I couldn't work out the lifetime problem that stopped me from just returning
    // `(categorise_hand(hand), hand)` and this is the easiest way I could think of to clone the hand vector :/
}

fn joker_sort_key(h: &(Hand, usize)) -> (HandType, Hand) {
    let (hand, _) = h;
    let mut jokerless: Vec<Card> = Vec::new();
    let mut jokerful: Vec<Card> = Vec::new();
    let mut num_jokers: usize = 0;
    for card in hand {
        match card {
            Card::Jack => {
                num_jokers += 1;
                jokerful.push(Card::Joker);
            }
            _ => {
                jokerless.push(*card);
                jokerful.push(*card);
            }
        }
    }
    let (sub_distinct, count) = get_distinct_and_count(&jokerless);
    let distinct = sub_distinct + (5 - jokerless.len());
    let mut hand_type = categorise_from_distinct_count(distinct, count);
    for _ in 0..num_jokers {
        hand_type = hand_type.upgrade();
    }
    (hand_type, jokerful)
}
