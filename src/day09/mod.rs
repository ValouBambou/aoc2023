use crate::DaySolution;

pub struct Day9;

impl DaySolution<i32> for Day9 {
    const DAY: u8 = 9;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: i32 = 114;
    const EXAMPLE_ANSWER_2: i32 = 2;

    fn solve_part1(input: &str) -> i32 {
        parse(input)
            .map(|seq| infer_next_term(&seq, *seq.last().unwrap()))
            .sum()
    }

    fn solve_part2(input: &str) -> i32 {
        parse(input)
            .map(|seq| infer_prev_term(&seq, *seq.first().unwrap()))
            .sum()
    }
}

fn infer_next_term(sequence: &[i32], last: i32) -> i32 {
    if sequence.iter().all(|&x| x == 0) {
        last
    } else {
        let difseq: Vec<i32> = sequence.windows(2).map(|pair| pair[1] - pair[0]).collect();
        last + infer_next_term(&difseq, *difseq.last().unwrap())
    }
}

fn infer_prev_term(sequence: &[i32], first: i32) -> i32 {
    if sequence.iter().all(|&x| x == 0) {
        first
    } else {
        let difseq: Vec<i32> = sequence.windows(2).map(|pair| pair[1] - pair[0]).collect();
        first - infer_prev_term(&difseq, *difseq.first().unwrap())
    }
}

fn parse(input: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    input.trim().lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    })
}
