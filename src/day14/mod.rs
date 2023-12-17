use crate::{array2d::Array2D, parse_utils, vec2d::Vec2D, DaySolution};

pub struct Day14;

impl DaySolution for Day14 {
    const DAY: u8 = 14;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: usize = 136;
    const EXAMPLE_ANSWER_2: usize = 64;

    fn solve_part1(input: &str) -> usize {
        let mut grid = parse_utils::char_grid(input);
        slide(&mut grid, NORTH);
        compute_load(&grid)
    }

    fn solve_part2(input: &str) -> usize {
        let mut grid = parse_utils::char_grid(input);
        let loads: Vec<usize> = (0..BUF_LEN).map(|_| cycle(&mut grid)).collect();
        let (cycle_start, cycle_len) = search_pattern(&loads);
        let offset = (TOTAL_CYCLES - cycle_start) % cycle_len;
        loads[cycle_start + offset - 1]
    }
}
const TOTAL_CYCLES: usize = 1_000_000_000;
const BUF_LEN: usize = 150; // need to be big enough

fn search_pattern(loads: &[usize]) -> (usize, usize) {
    for len in 2..BUF_LEN {
        for start in 0..(BUF_LEN - 2 * len) {
            let pattern = &loads[start..(start + len)];
            if ((start + len)..(BUF_LEN - len))
                .step_by(len)
                .all(|start_i| {
                    let slice_i = &loads[start_i..(start_i + len)];
                    slice_i == pattern
                })
            {
                return (start, len);
            }
        }
    }
    panic!("Pattern not found");
}

fn compute_load(grid: &Array2D<char>) -> usize {
    let h = grid.height();
    let mut res = 0;
    for y in 0..h {
        for x in 0..grid.width() {
            if grid.get(x, y) == 'O' {
                res += h - y;
            }
        }
    }
    res
}

fn cycle(grid: &mut Array2D<char>) -> usize {
    slide(grid, NORTH);
    slide(grid, WEST);
    slide(grid, SOUTH);
    slide(grid, EAST);
    compute_load(&grid)
}
const NORTH: Vec2D = Vec2D { x: 0, y: -1 };
const SOUTH: Vec2D = Vec2D { x: 0, y: 1 };
const EAST: Vec2D = Vec2D { x: 1, y: 0 };
const WEST: Vec2D = Vec2D { x: -1, y: 0 };

// fn slide(grid: &mut Array2D<char>, direction: Vec2D) {
//     assert_eq!(direction.x.abs() + direction.y.abs(), 1);
//     let dims = grid.dimensions();
//     let (end, other_axis) = if direction.x != 0 {
//         (grid.width(), 1)
//     } else {
//         (grid.height(), 0)
//     };
//     let mut cur: Vec2D = direction * dims - 1;
//     if cur.x < 0 {
//         cur.x = 0;
//     }
//     if cur.y < 0 {
//         cur.y = 0;
//     }
//
//     for moves in 1..end {
//         cur -= direction * moves;
//         for _ in 0..moves {
//             let mut next = cur + direction;
//             for _ in 0..dims[other_axis] {
//                 if grid.getv(next) == '.' && grid.getv(cur) == 'O' {
//                     grid.setv(next, 'O');
//                     grid.setv(cur, '.');
//                 }
//                 cur[other_axis] += 1;
//                 next[other_axis] += 1;
//             }
//             cur += direction;
//             cur[other_axis] = 0;
//         }
//     }
// }

fn slide(grid: &mut Array2D<char>, direction: Vec2D) {
    let dims = grid.dimensions();
    let (start, other_axis) = match direction {
        NORTH => (
            Vec2D {
                x: 0,
                y: dims[1] - 1,
            },
            0,
        ),
        WEST => (
            Vec2D {
                x: dims[0] - 1,
                y: 0,
            },
            1,
        ),
        SOUTH => (Vec2D { x: 0, y: 0 }, 0),
        EAST => (Vec2D { x: 0, y: 0 }, 1),
        _ => panic!("wtf"),
    };
    for i in 0..dims[other_axis] {
        let mut cur = start;
        cur[other_axis] += i;
        slide_column(grid, cur, direction, dims[1 - other_axis]);
    }
}

fn slide_column(grid: &mut Array2D<char>, mut cur: Vec2D, direction: Vec2D, len: i64) {
    let mut count_rocks = 0;
    for _ in 0..len {
        match grid.getv(cur) {
            'O' => {
                count_rocks += 1;
                grid.setv(cur, '.');
            }
            '#' => {
                let mut tmp = cur - direction;
                for _ in 0..count_rocks {
                    grid.setv(tmp, 'O');
                    tmp -= direction;
                }
                count_rocks = 0;
            }
            _ => (),
        }
        cur += direction;
    }
    // dont forget the final wall invisible but still important
    let mut tmp = cur - direction;
    for _ in 0..count_rocks {
        grid.setv(tmp, 'O');
        tmp -= direction;
    }
}
