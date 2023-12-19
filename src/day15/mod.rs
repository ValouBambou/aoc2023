use crate::DaySolution;

pub struct Day15;

impl DaySolution for Day15 {
    const DAY: u8 = 15;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: usize = 1320;
    const EXAMPLE_ANSWER_2: usize = 145;

    fn solve_part1(input: &str) -> usize {
        input.trim().split(',').map(ascii_hash).sum()
    }

    fn solve_part2(input: &str) -> usize {
        let mut boxes: Vec<Vec<&str>> = vec![vec![]; 256];
        input.trim().split(',').for_each(|instr| {
            if instr.ends_with('-') {
                let label = instr.trim_end_matches('-');
                let box_i = ascii_hash(label);
                let idx_rm = boxes[box_i].iter().position(|l| l.starts_with(label));
                if let Some(i) = idx_rm {
                    boxes[box_i].remove(i);
                }
            } else {
                let label = instr.split_once('=').unwrap().0;
                let box_i = ascii_hash(label);
                let idx_rm = boxes[box_i].iter().position(|l| l.starts_with(label));
                match idx_rm {
                    Some(i) => {
                        boxes[box_i][i] = instr;
                    }
                    None => {
                        boxes[box_i].push(instr);
                    }
                }
            }
        });
        boxes
            .into_iter()
            .enumerate()
            .flat_map(move |(i, lens)| {
                lens.into_iter().enumerate().map(move |(j, instr)| {
                    let focal = instr.chars().last().unwrap().to_digit(10).unwrap();
                    (i + 1) * (j + 1) * (focal as usize)
                })
            })
            .sum()
    }
}

fn ascii_hash(input: &str) -> usize {
    input
        .chars()
        .map(|c| c as usize)
        .fold(0, |acc, ascii| ((acc + ascii) * 17) % 256)
}
