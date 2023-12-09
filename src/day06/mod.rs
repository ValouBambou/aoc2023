use crate::DaySolution;

pub struct Day6;

impl DaySolution for Day6 {
    const DAY: u8 = 6;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: usize = 288;
    const EXAMPLE_ANSWER_2: usize = 71503;

    fn solve_part1(input: &str) -> usize {
        let mut lines = input.trim().lines().map(|s| {
            s.split_once(':')
                .unwrap()
                .1
                .trim_start()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
        });
        lines
            .next()
            .unwrap()
            .zip(lines.next().unwrap())
            .into_iter()
            .map(|(time, dist)| {
                (1..time)
                    .filter(|duration| duration * (time - duration) > dist)
                    .count()
            })
            .product()
    }
    fn solve_part2(input: &str) -> usize {
        let mut lines = input.trim().lines().map(|s| {
            let s = s.split_once(':').unwrap().1.replace(' ', "");
            s.parse::<usize>().unwrap()
        });
        let time = lines.next().unwrap() as f64;
        let dist = lines.next().unwrap() as f64;
        let duration_max = (time + (time * time - 4.0 * dist).sqrt()) / 2.0;
        let duration_min = (time - (time * time - 4.0 * dist).sqrt()) / 2.0;
        (duration_max.ceil() - duration_min.ceil()) as usize
    }
}
