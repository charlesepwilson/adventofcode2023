use std::collections::HashSet;
use crate::utils::read_lines;

pub fn part1() -> usize {
    let mut input = parse_input();
    input.sort_by_cached_key(sort_key);
    let winnings = input.iter().enumerate().map(|(rank, (_, bid))| bid * (rank + 1));
    winnings.sum()
}
pub fn part2() -> usize {
    // let input = parse_input();
    0
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
enum Card {
    Two = 2,
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
    HighCard,
    Pair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
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
    let card_set: HashSet<_> = hand.iter().map(|x| *x).collect();
    let distinct = card_set.len();
    match distinct {
        5 => return HandType::HighCard,
        4 => return HandType::Pair,
        1 => return HandType::FiveOfKind,
        _ => (),
    }
    let mut counts = Vec::new();
    for card in hand {
        counts.push(hand.iter().filter(|&x| x == card).count());
    }
    let max_count = counts.iter().max().unwrap();
    match (distinct, max_count) {
        (_, 2) => HandType::TwoPair,
        (_, 4) => HandType::FourOfKind,
        (3, 3) => HandType::ThreeOfKind,
        (2, 3) => HandType::FullHouse,
        _ => panic!(),
    }

}

fn sort_key(h: &(Hand, usize)) -> (HandType, Hand) {
    let (hand, _) = h;
    (categorise_hand(hand), hand.iter().map(|x| *x).collect())
    // This is bad...I couldn't work out the lifetime problem that stopped me from just returning
    // `(categorise_hand(hand), hand)` and this is the easiest way I could think of to clone the hand vector :/
}
