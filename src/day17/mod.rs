use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::{array2d::Array2D, parse_utils, vec2d::Vec2D, DaySolution};

pub struct Day17;

impl DaySolution for Day17 {
    const DAY: u8 = 17;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: usize = 102;
    const EXAMPLE_ANSWER_2: usize = 94;

    fn solve_part1(input: &str) -> usize {
        let grid = parse_utils::digits_grid(input);
        let start = Vec2D { x: 0, y: 0 };
        let end = grid.dimensions() - 1;
        shortest_path(&grid, start, end, 1, 3)
    }

    fn solve_part2(input: &str) -> usize {
        let grid = parse_utils::digits_grid(input);
        let start = Vec2D { x: 0, y: 0 };
        let end = grid.dimensions() - 1;
        shortest_path(&grid, start, end, 4, 10)
    }
}
fn shortest_path(
    grid: &Array2D<u8>,
    start: Vec2D,
    end: Vec2D,
    n_min: usize,
    n_max: usize,
) -> usize {
    let mut dist = HashMap::new();
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, start, 0, Vec2D { x: 1, y: 0 })));
    while let Some(Reverse((cur_dist, cur, n, dir))) = pq.pop() {
        let dirs_n = [
            (dir, n + 1),
            (dir.rotate90_left(), 1),
            (dir.rotate90_right(), 1),
        ];
        for (new_dir, mut new_n) in dirs_n {
            if new_n > n_max {
                continue;
            }
            let mut nei = cur + new_dir;
            if !grid.is_valid_idxv(nei) {
                continue;
            }
            let mut new_dist = cur_dist + grid.getv(nei) as usize;
            while new_n < n_min {
                nei += new_dir;
                if !grid.is_valid_idxv(nei) {
                    break;
                }
                new_dist += grid.getv(nei) as usize;
                new_n += 1;
            }
            if !grid.is_valid_idxv(nei) {
                continue;
            }

            let new_tmp = (nei, new_n, new_dir);
            if new_dist < dist.get(&new_tmp).cloned().unwrap_or(usize::MAX) {
                dist.insert(new_tmp, new_dist);
                pq.push(Reverse((new_dist, nei, new_n, new_dir)));
            }
        }
    }
    dist.into_iter()
        .filter(|(k, _)| k.0 == end)
        .map(|(_, v)| v)
        .min()
        .unwrap()
}
