use crate::DaySolution;

pub struct Day5;
impl DaySolution<u64> for Day5 {
    const DAY: u8 = 5;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: u64 = 35;
    const EXAMPLE_ANSWER_2: u64 = 46;

    fn solve_part1(input: &str) -> u64 {
        let (seeds, mappers) = parse(input);
        seeds
            .into_iter()
            .map(|seed| mappers.iter().fold(seed, |acc, mapper| mapper.convert(acc)))
            .min()
            .unwrap()
    }

    fn solve_part2(input: &str) -> u64 {
        let (seeds, mappers) = parse(input);
        seeds
            .windows(2)
            .step_by(2)
            .map(|window| {
                let start = window[0];
                let len = window[1];
                find_min_in_range(start, len, 0, &mappers)
            })
            .min()
            .unwrap()
    }
}

#[derive(Debug)]
struct CategoryMapper {
    map_range: Vec<(u64, u64, u64)>,
}

impl CategoryMapper {
    fn convert(&self, num: u64) -> u64 {
        self.map_range
            .iter()
            .find_map(|&(dst_start, src_start, len)| {
                (src_start..(src_start + len)).contains(&num).then(|| {
                    let dif = num - src_start;
                    dst_start + dif
                })
            })
            .unwrap_or(num)
    }
    fn convert_range(&self, start: u64, len: u64) -> Vec<(u64, u64)> {
        // sub interval mapped, destination start range
        // println!("start = {start} len = {len}");
        let mut intersections: Vec<((u64, u64), u64)> = self
            .map_range
            .iter()
            .filter_map(|&(dst_start, src_start, it_len)| {
                intersection((start, len), (src_start, it_len)).map(|sub_interval| {
                    let dif = sub_interval.0 - src_start;
                    (sub_interval, dst_start + dif)
                })
            })
            .collect();
        intersections.sort();

        // some part of the interval may be unmapped so it should be mapped with identity
        let rest: Vec<((u64, u64), u64)> = intersections
            .windows(2)
            .filter_map(|window| {
                let (range_left, _) = window[0];
                let (range_right, _) = window[1];
                let end_left = range_left.0 + range_left.1;
                let dif = range_right.0 - end_left;
                (dif > 0).then_some(((end_left, dif), end_left))
            })
            .collect();

        match intersections.last().cloned() {
            Some(((i_start, i_len), _)) if i_start + i_len < start + len => {
                let last_start = i_start + i_len;
                intersections.push(((last_start, start + len - last_start), last_start));
            }
            None => {
                intersections.push(((start, len), start));
            }
            _ => (),
        }

        match intersections.first().cloned() {
            Some(((i_start, _), _)) if i_start > start => {
                intersections.push(((start, i_start - start), start));
            }
            _ => (),
        }
        intersections.extend(rest);
        intersections
            .into_iter()
            .map(|((_, range_len), dst_start)| (dst_start, range_len))
            .collect()
    }
    fn from_str(s: &str) -> Self {
        let map_range: Vec<(u64, u64, u64)> = s
            .lines()
            .skip(1)
            .map(|line| {
                let mut ints = line.split_ascii_whitespace().map(|x| x.parse().unwrap());
                (
                    ints.next().unwrap(),
                    ints.next().unwrap(),
                    ints.next().unwrap(),
                )
            })
            .collect();
        Self { map_range }
    }
}

fn parse(input: &str) -> (Vec<u64>, Vec<CategoryMapper>) {
    let mut blocks = input.trim().split("\n\n");
    // first line is like this "seeds: 79 14 55 13"
    let seeds: Vec<u64> = blocks
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mappers: Vec<CategoryMapper> = blocks.map(CategoryMapper::from_str).collect();
    (seeds, mappers)
}

fn find_min_in_range(start: u64, len: u64, cur_map: usize, mappers: &[CategoryMapper]) -> u64 {
    if cur_map >= mappers.len() {
        start
    } else {
        mappers[cur_map]
            .convert_range(start, len)
            .into_iter()
            .map(|(new_start, new_len)| find_min_in_range(new_start, new_len, cur_map + 1, mappers))
            .min()
            .unwrap()
    }
}

/// tuple are start, len
fn intersection(interval1: (u64, u64), interval2: (u64, u64)) -> Option<(u64, u64)> {
    let max_1 = interval1.0 + interval1.1;
    let max_2 = interval2.0 + interval2.1;
    let inter_max = max_1.min(max_2);
    let inter_min = interval1.0.max(interval2.0);
    (inter_min < inter_max).then_some((inter_min, inter_max.saturating_sub(inter_min)))
}
