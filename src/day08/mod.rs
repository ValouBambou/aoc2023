use std::collections::HashMap;

use crate::{math_utils, DaySolution};

pub struct Day8;

impl DaySolution for Day8 {
    const DAY: u8 = 8;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: usize = 6;
    const EXAMPLE_ANSWER_2: usize = 6;

    fn solve_part1(input: &str) -> usize {
        let (instructions, nodes_map, indices) = parse(input);
        let start = *indices.get("AAA").unwrap();
        let end = *indices.get("ZZZ").unwrap();
        let mut cur = start;
        instructions
            .into_iter()
            .cycle()
            .enumerate()
            .find_map(|(step, instruction)| {
                let res = (cur == end).then_some(step);
                cur = nodes_map[cur][instruction];
                res
            })
            .unwrap()
    }

    fn solve_part2(input: &str) -> usize {
        let (instructions, nodes_map, indices) = parse(input);
        let start: Vec<usize> = indices
            .iter()
            .filter_map(|(k, &v)| k.ends_with('A').then_some(v))
            .collect();

        let end: Vec<usize> = indices
            .iter()
            .filter_map(|(k, &v)| k.ends_with('Z').then_some(v))
            .collect();
        start
            .into_iter()
            .map(|start_node| {
                let mut cur = start_node;
                instructions
                    .iter()
                    .cycle()
                    .enumerate()
                    .find_map(|(i, &instr)| {
                        let res = end.contains(&cur).then_some(i);
                        cur = nodes_map[cur][instr];
                        res
                    })
                    .unwrap()
            })
            .fold(1, math_utils::lcm)
    }
}

fn parse(input: &str) -> (Vec<usize>, Vec<[usize; 2]>, HashMap<&str, usize>) {
    let mut lines = input.trim().lines();
    let left_rights: Vec<usize> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| (c == 'R') as usize)
        .collect();
    let lines = lines.skip(1).map(|line| line.split_once(" = ").unwrap());
    let indices: HashMap<&str, usize> = lines
        .clone()
        .enumerate()
        .map(|(i, (name, _))| (name, i))
        .collect();
    let nodes_map: Vec<[usize; 2]> = lines
        .map(|(_, tuple)| {
            let (left, right) = tuple
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(", ")
                .unwrap();
            [*indices.get(left).unwrap(), *indices.get(right).unwrap()]
        })
        .collect();
    (left_rights, nodes_map, indices)
}
