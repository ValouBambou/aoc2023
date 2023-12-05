use crate::{array2d::Array2D, DaySolution};

pub struct Day3;

impl DaySolution<u32> for Day3 {
    const DAY: u8 = 3;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: u32 = 4361;
    const EXAMPLE_ANSWER_2: u32 = 467835;

    fn solve_part1(input: &str) -> u32 {
        let numbers = parse_numbers(input);
        input
            .trim()
            .lines()
            .enumerate()
            .flat_map(move |(i, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(j, c)| (c != '.' && (!c.is_ascii_digit())).then_some((j, i)))
            })
            .map(|(x, y)| {
                let neighbors = numbers.neighbors8_indexed(x, y);
                // we cant just sum because some number may be counted twice
                // example:
                // 123
                // .*.
                // here 123 will be summed 3 times
                neighbors.iter().fold(0, |acc, &(x, y, num)| {
                    if neighbors.contains(&(x + 1, y, num)) {
                        acc
                    } else {
                        acc + num
                    }
                })
            })
            .sum()
    }

    fn solve_part2(input: &str) -> u32 {
        let numbers = parse_numbers(input);
        input
            .trim()
            .lines()
            .enumerate()
            .flat_map(move |(i, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(j, c)| (c == '*').then_some((j, i)))
            })
            .filter_map(|(x, y)| {
                let neighbors = numbers.neighbors8_indexed(x, y);
                let unique_neighbors: Vec<u32> = neighbors
                    .iter()
                    .filter_map(|&(x, y, num)| {
                        (num != 0 && !neighbors.contains(&(x + 1, y, num))).then_some(num)
                    })
                    .collect();
                (unique_neighbors.len() == 2).then(|| unique_neighbors[0] * unique_neighbors[1])
            })
            .sum()
    }
}

fn parse_numbers(input: &str) -> Array2D<u32> {
    let input = input.trim();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut res = Array2D::<u32>::new_default(width, height);
    for (y, line) in input.lines().enumerate() {
        let mut numbers: Vec<(usize, u32)> = vec![];
        for (x, c) in line.chars().enumerate() {
            let d = if let Some(d) = c.to_digit(10) {
                d
            } else {
                continue;
            };
            // accumulate consecutives digits to a single number
            match numbers.last_mut() {
                Some((prev_j, num)) if *prev_j == x - 1 => {
                    *num *= 10;
                    *num += d;
                    *prev_j = x;
                }
                _ => numbers.push((x, d)),
            }
        }
        for (j_last_digit, number) in numbers {
            let len = 1 + number.ilog10() as usize;
            let x = j_last_digit + 1 - len;
            for i in 0..len {
                res.set(x + i, y, number);
            }
        }
    }
    res
}
