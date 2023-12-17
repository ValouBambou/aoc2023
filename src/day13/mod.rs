use crate::{array2d::Array2D, parse_utils, DaySolution};

pub struct Day13;

impl DaySolution for Day13 {
    const DAY: u8 = 13;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: usize = 405;
    const EXAMPLE_ANSWER_2: usize = 400;

    fn solve_part1(input: &str) -> usize {
        solve(input, is_symmetric)
    }

    fn solve_part2(input: &str) -> usize {
        solve(input, get_smudge)
    }
}
fn solve<F: Fn(usize, usize, &Array2D<char>) -> bool>(input: &str, predicate: F) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(parse_utils::char_grid)
        .map(|grid| process(grid, &predicate))
        .sum()
}

fn process<F: Fn(usize, usize, &Array2D<char>) -> bool>(
    grid: Array2D<char>,
    predicate: F,
) -> usize {
    let sym_col = (0..(grid.width() - 1))
        .find(|&x| predicate(x, 0, &grid))
        .map(|x| x + 1);
    if let Some(n) = sym_col {
        return n;
    }
    100 * (0..(grid.height() - 1))
        .find(|&y| predicate(y, 1, &grid))
        .map(|y| y + 1)
        .unwrap_or_default()
}

fn is_symmetric(idx: usize, axis: usize, grid: &Array2D<char>) -> bool {
    assert!(axis <= 1);
    let dims = [grid.width(), grid.height()];
    let mut left = [0, 0];
    left[axis] = idx;
    let mut right = [0, 0];
    right[axis] = idx + 1;

    while (0..dims[1 - axis]).all(|col| {
        left[1 - axis] = col;
        right[1 - axis] = col;
        grid.get(left[0], left[1]) == grid.get(right[0], right[1])
    }) {
        if left[axis] == 0 || right[axis] == dims[axis] - 1 {
            return true;
        }
        left[axis] -= 1;
        right[axis] += 1;
    }
    false
}

fn get_smudge(idx: usize, axis: usize, grid: &Array2D<char>) -> bool {
    assert!(axis <= 1);
    let dims = [grid.width(), grid.height()];
    let mut left = [0, 0];
    left[axis] = idx;
    let mut right = [0, 0];
    right[axis] = idx + 1;

    let mut errors = 0;
    let mut border_reached = false;

    while errors < 2 {
        let mut smudges = (0..dims[1 - axis]).filter(|&col| {
            left[1 - axis] = col;
            right[1 - axis] = col;
            grid.get(left[0], left[1]) != grid.get(right[0], right[1])
        });
        errors += match (smudges.next(), smudges.next()) {
            (Some(_), None) => 1,
            (None, None) => 0,
            _ => 2,
        };

        border_reached = left[axis] == 0 || right[axis] == dims[axis] - 1;
        if border_reached {
            break;
        }
        left[axis] -= 1;
        right[axis] += 1;
    }
    errors == 1 && border_reached
}
