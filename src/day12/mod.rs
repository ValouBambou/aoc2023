use crate::DaySolution;

pub struct Day12;

impl DaySolution for Day12 {
    const DAY: u8 = 12;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: usize = 21;
    const EXAMPLE_ANSWER_2: usize = 525152;

    fn solve_part1(input: &str) -> usize {
        solve(input, 1)
    }

    fn solve_part2(input: &str) -> usize {
        solve(input, 5)
    }
}

// credit to https://www.reddit.com/r/adventofcode/comments/18ge41g/comment/kdnh1u5/
// and the original code https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day12.rs
fn solve(input: &str, repeat: usize) -> usize {
    let lines = parse(input);
    let mut pattern = Vec::new();
    let mut springs = Vec::new();
    // Exact size is not too important as long as there's enough space.
    let mut broken = vec![0; 200];
    let mut table = vec![0; 200 * 50];
    lines
        .map(|(gears, groups)| {
            pattern.clear();
            springs.clear();
            // handle twist of part 2
            for _ in 1..repeat {
                pattern.extend_from_slice(gears);
                pattern.push(b'?');
                springs.extend_from_slice(&groups);
            }
            // handle regular stuf
            pattern.extend_from_slice(gears);
            pattern.push(b'.'); // trick to avoid some +1 or -1 in index/len offsets
            springs.extend_from_slice(&groups);
            // margin for springs
            let wiggle = pattern.len() - springs.iter().sum::<usize>() - springs.len() + 1;
            // cummulative sum to catch easily if spring can fit in a range for latter
            let mut sum = 0;
            broken.push(0);
            for (i, &b) in pattern.iter().enumerate() {
                if b != b'.' {
                    sum += 1;
                }
                broken[i + 1] = sum;
            }

            let size = springs[0];
            let mut sum = 0;
            let mut valid = true;

            // special case for the first row of the table.
            for i in 0..wiggle {
                // check for right #
                if pattern[i + size] == b'#' {
                    sum = 0;
                } else if valid && broken[i + size] - broken[i] == size {
                    sum += 1;
                }
                table[i + size] = sum;
                // check for left #
                valid &= pattern[i] != b'#';
            }

            // cummulative sum using the previous row with index -1 to accumulate
            let mut start = size + 1;
            for (row, &size) in springs.iter().enumerate().skip(1) {
                let previous = (row - 1) * pattern.len();
                let current = row * pattern.len();
                sum = 0; // reset per row
                for i in start..start + wiggle {
                    // As a minor optimization only check the pattern if the previous row
                    // will contribute a non-zero value.
                    if pattern[i + size] == b'#' {
                        sum = 0;
                    } else if table[previous + i - 1] > 0
                        && pattern[i - 1] != b'#'
                        && broken[i + size] - broken[i] == size
                    {
                        sum += table[previous + i - 1];
                    }

                    table[current + i + size] = sum;
                }
                start += size + 1;
            }
            // only the last value in the array (bottom right is valuable)
            sum
        })
        .sum()
}

fn parse(input: &str) -> impl Iterator<Item = (&'_ [u8], Vec<usize>)> {
    input.trim().lines().map(|line| {
        let (gears, groups) = line.split_once(' ').unwrap();
        (
            gears.as_bytes(),
            groups.split(',').map(|x| x.parse().unwrap()).collect(),
        )
    })
}
