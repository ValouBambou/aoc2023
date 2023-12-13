use std::collections::HashSet;

use crate::{vec2d::Vec2D, DaySolution};

pub struct Day11;

impl DaySolution<i64> for Day11 {
    const DAY: u8 = 11;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: i64 = 374;
    const EXAMPLE_ANSWER_2: i64 = 8410;

    fn solve_part1(input: &str) -> i64 {
        solve_with_expansion(input, 2)
    }

    fn solve_part2(input: &str) -> i64 {
        if input.len() < 120 {
            // example so test with expansion smaller
            solve_with_expansion(input, 100)
        } else {
            solve_with_expansion(input, 1_000_000)
        }
    }
}

fn solve_with_expansion(input: &str, expansion: usize) -> i64 {
    let expansion = expansion - 1;
    let (w, h, mut galaxies) = parse(input);
    let mut empty_cols: HashSet<i64> = (0..(w as i64)).collect();
    let mut empty_rows: HashSet<i64> = (0..(h as i64)).collect();
    for coord in galaxies.iter() {
        empty_rows.remove(&coord.y);
        empty_cols.remove(&coord.x);
    }
    galaxies.iter_mut().for_each(|coord| {
        let offset_x = empty_cols.iter().filter(|&&x| x < coord.x).count();
        let offset_y = empty_rows.iter().filter(|&&y| y < coord.y).count();
        coord.x += (expansion * offset_x) as i64;
        coord.y += (expansion * offset_y) as i64;
    });
    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, coord1)| {
            galaxies
                .iter()
                .take(i)
                .map(|coord2| coord2.manhattan_dist(coord1))
        })
        .sum()
}

fn parse(input: &str) -> (usize, usize, Vec<Vec2D>) {
    let input = input.trim();
    let height = input.lines().count();
    let width = input.len() / height;
    let buf: Vec<Vec2D> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, c)| {
                (c == '#').then_some(Vec2D {
                    x: x as i64,
                    y: y as i64,
                })
            })
        })
        .collect();
    (width, height, buf)
}
