use std::collections::{HashSet, VecDeque};

use crate::{array2d::Array2D, parse_utils, vec2d::Vec2D, DaySolution};

pub struct Day16;

impl DaySolution for Day16 {
    const DAY: u8 = 16;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: usize = 46;
    const EXAMPLE_ANSWER_2: usize = 51;

    fn solve_part1(input: &str) -> usize {
        let grid = parse_utils::char_grid(input);
        let start = Vec2D { x: 0, y: 0 };
        let start_dir = Vec2D { x: 1, y: 0 };
        energetised_tiles(&grid, start, start_dir)
    }

    fn solve_part2(input: &str) -> usize {
        let grid = parse_utils::char_grid(input);
        let (w, h) = (grid.width() as i64, grid.height() as i64);
        let mut max = 0;
        for x in 0..w {
            let start1 = Vec2D { x, y: 0 };
            let dir1 = Vec2D { x: 0, y: 1 };
            let start2 = Vec2D { x, y: h - 1 };
            let dir2 = Vec2D { x: 0, y: -1 };
            max = max.max(energetised_tiles(&grid, start1, dir1));
            max = max.max(energetised_tiles(&grid, start2, dir2));
        }
        for y in 0..h {
            let start1 = Vec2D { x: 0, y };
            let dir1 = Vec2D { x: 1, y: 0 };
            let start2 = Vec2D { x: w - 1, y };
            let dir2 = Vec2D { x: -1, y: 0 };
            max = max.max(energetised_tiles(&grid, start1, dir1));
            max = max.max(energetised_tiles(&grid, start2, dir2));
        }
        max
    }
}

fn energetised_tiles(grid: &Array2D<char>, start: Vec2D, start_dir: Vec2D) -> usize {
    let mut queue = VecDeque::from([(start, start_dir)]);
    let mut visited = HashSet::from([(start, start_dir)]);
    while let Some((cur, dir)) = queue.pop_front() {
        let nexts = match grid.getv(cur) {
            '|' if dir.x != 0 => [
                Some(cur + dir.rotate90_left()),
                Some(cur + dir.rotate90_right()),
            ],
            '-' if dir.y != 0 => [
                Some(cur + dir.rotate90_left()),
                Some(cur + dir.rotate90_right()),
            ],
            '/' if dir.x != 0 => [Some(cur + dir.rotate90_left()), None],
            '/' if dir.y != 0 => [Some(cur + dir.rotate90_right()), None],
            '\\' if dir.x != 0 => [Some(cur + dir.rotate90_right()), None],
            '\\' if dir.y != 0 => [Some(cur + dir.rotate90_left()), None],
            _ => [Some(cur + dir), None],
        };
        nexts.into_iter().filter_map(|x| x).for_each(|next| {
            let new_tup = (next, next - cur);
            if grid.is_valid_idxv(next) && (!visited.contains(&new_tup)) {
                visited.insert(new_tup);
                queue.push_back(new_tup);
            }
        });
    }
    let visited: HashSet<Vec2D> = visited.into_iter().map(|x| x.0).collect();
    visited.len()
}
