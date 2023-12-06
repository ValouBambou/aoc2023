use crate::DaySolution;

pub struct Day4;

impl DaySolution<u32> for Day4 {
    const DAY: u8 = 4;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: u32 = 13;
    const EXAMPLE_ANSWER_2: u32 = 30;

    fn solve_part1(input: &str) -> u32 {
        input
            .trim()
            .lines()
            .map(count_match_wins)
            .map(|n| (1 << n) / 2)
            .sum()
    }

    fn solve_part2(input: &str) -> u32 {
        let match_per_cards: Vec<u32> = input.trim().lines().map(count_match_wins).collect();
        let mut counts_cards = vec![1u32; match_per_cards.len()];
        let idx_max = match_per_cards.len() - 1;
        match_per_cards
            .into_iter()
            .enumerate()
            .for_each(|(i, matchs)| {
                let max_i = idx_max.min(i + matchs as usize);
                for j in (i + 1)..=max_i {
                    counts_cards[j] += counts_cards[i];
                }
            });
        counts_cards.into_iter().sum()
    }
}

fn count_match_wins(line: &str) -> u32 {
    // parse the 2 numbers list
    let (_, lists) = line.split_once(": ").unwrap();
    let (win_nums, nums) = lists.split_once(" | ").unwrap();
    let mut win_nums: Vec<u32> = win_nums
        .split_ascii_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let mut nums: Vec<u32> = nums
        .split_ascii_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    win_nums.sort();
    nums.sort();
    // nlog(n) instead of n squared
    // no hashset since for this small number of n it is not worth the overhead

    // count match
    let (mut wi, mut i, mut count) = (0, 0, 0);
    while wi < win_nums.len() && i < nums.len() {
        if nums[i] < win_nums[wi] {
            i += 1;
        } else if nums[i] == win_nums[wi] {
            i += 1;
            wi += 1;
            count += 1;
        } else {
            wi += 1;
        }
    }
    count
}
