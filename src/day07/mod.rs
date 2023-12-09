use std::cmp::Ordering;

use crate::DaySolution;

pub struct Day7;

impl DaySolution for Day7 {
    const DAY: u8 = 7;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: usize = 6440;
    const EXAMPLE_ANSWER_2: usize = 5905;

    fn solve_part1(input: &str) -> usize {
        let mut hands_bids = parse(input);
        hands_bids.sort_by(|(hand1, _), (hand2, _)| compare_hands(hand1, hand2));
        hands_bids
            .into_iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) * bid)
            .sum()
    }

    fn solve_part2(input: &str) -> usize {
        let mut hands_bids = parse(input);
        hands_bids.sort_by(|(hand1, _), (hand2, _)| compare_hands2(hand1, hand2));
        hands_bids
            .into_iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) * bid)
            .sum()
    }
}
const HAND: usize = 5;

fn parse(input: &str) -> Vec<([char; HAND], usize)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (hand, bid) = line.trim_end().split_once(' ').unwrap();
            let bid: usize = bid.parse().unwrap();
            let mut chars = hand.chars();
            let hand = [
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
            ];
            (hand, bid)
        })
        .collect()
}

fn card_value(a: &char) -> usize {
    match a {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("WTF unknown card {a}"),
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    Five = 6,
    Four = 5,
    Full = 4,
    Three = 3,
    TwoPairs = 2,
    OnePair = 1,
    Cards = 0,
}

fn hand_type(hand: &[char; HAND]) -> HandType {
    let mut count = [0; 13];
    for card in hand {
        let idx = card_value(card);
        count[idx] += 1;
    }
    count.sort();
    match (count[12], count[11]) {
        (5, _) => HandType::Five,
        (4, _) => HandType::Four,
        (3, 2) => HandType::Full,
        (3, _) => HandType::Three,
        (2, 2) => HandType::TwoPairs,
        (2, 1) => HandType::OnePair,
        (1, 1) => HandType::Cards,
        _ => panic!("wtf too few cards"),
    }
}

fn compare_hands(hand1: &[char; HAND], hand2: &[char; HAND]) -> Ordering {
    match hand_type(hand1).cmp(&hand_type(hand2)) {
        Ordering::Equal => {
            let mut i = 0;
            while i < HAND && hand1[i] == hand2[i] {
                i += 1;
            }
            if i == HAND {
                Ordering::Equal
            } else {
                card_value(&hand1[i]).cmp(&card_value(&hand2[i]))
            }
        }
        otherwise => otherwise,
    }
}

fn card_value2(a: &char) -> usize {
    match a {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("WTF unknown card {a}"),
    }
}

fn hand_type2(hand: &[char; HAND]) -> HandType {
    let mut count = [0; 13];
    for card in hand {
        let idx = card_value2(card);
        count[idx] += 1;
    }
    let n_jokers = count[0];
    count[0] = 0;
    let (idx_max, _) = count
        .iter()
        .enumerate()
        .max_by(|a, b| (a.1, a.0).cmp(&(b.1, b.0)))
        .unwrap();
    count[idx_max] += n_jokers;
    count.sort();
    match (count[12], count[11]) {
        (5, _) => HandType::Five,
        (4, _) => HandType::Four,
        (3, 2) => HandType::Full,
        (3, _) => HandType::Three,
        (2, 2) => HandType::TwoPairs,
        (2, 1) => HandType::OnePair,
        (1, 1) => HandType::Cards,
        _ => panic!("wtf too few cards"),
    }
}

fn compare_hands2(hand1: &[char; HAND], hand2: &[char; HAND]) -> Ordering {
    match hand_type2(hand1).cmp(&hand_type2(hand2)) {
        Ordering::Equal => {
            let mut i = 0;
            while i < HAND && hand1[i] == hand2[i] {
                i += 1;
            }
            if i == HAND {
                Ordering::Equal
            } else {
                card_value2(&hand1[i]).cmp(&card_value2(&hand2[i]))
            }
        }
        otherwise => otherwise,
    }
}
