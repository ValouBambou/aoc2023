use crate::DaySolution;

pub struct Day2;

impl DaySolution for Day2 {
    const DAY: u8 = 2;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: usize = 8;
    const EXAMPLE_ANSWER_2: usize = 2286;

    fn solve_part1(input: &str) -> usize {
        input
            .trim()
            .lines()
            .filter_map(|line| {
                let (game, cubes) = line.rsplit_once(": ").unwrap();
                let game: usize = game.rsplit_once(' ').unwrap().1.parse().unwrap();
                cubes
                    .split("; ")
                    .all(|subset| {
                        subset.split(", ").all(|count_color| {
                            let (count, color) = count_color.rsplit_once(' ').unwrap();
                            let count: u32 = count.parse().unwrap();
                            let max_count = match color {
                                "red" => 12,
                                "green" => 13,
                                "blue" => 14,
                                _ => panic!("wtf color = {color}"),
                            };
                            max_count >= count
                        })
                    })
                    .then_some(game)
            })
            .sum()
    }

    fn solve_part2(input: &str) -> usize {
        input
            .trim()
            .lines()
            .map(|line| {
                let cubes = line.rsplit_once(": ").unwrap().1;
                let mut min_colors = [1, 1, 1];
                cubes.split("; ").for_each(|subset| {
                    subset.split(", ").for_each(|count_color| {
                        let (count, color) = count_color.rsplit_once(' ').unwrap();
                        let count: usize = count.parse().unwrap();
                        let idx = match color {
                            "red" => 0,
                            "green" => 1,
                            "blue" => 2,
                            _ => panic!("wtf color = {color}"),
                        };
                        min_colors[idx] = min_colors[idx].max(count);
                    })
                });
                min_colors.into_iter().product::<usize>()
            })
            .sum()
    }
}
