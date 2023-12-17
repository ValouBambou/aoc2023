use crate::DaySolution;

pub struct Day1;

impl DaySolution<u32> for Day1 {
    const DAY: u8 = 1;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: u32 = 142;
    const EXAMPLE_ANSWER_2: u32 = 281;

    fn solve_part1(input: &str) -> u32 {
        input
            .trim()
            .lines()
            .map(|line| {
                let mut digits = line.chars().filter_map(|c| c.to_digit(10));
                let first_digit = digits.next().unwrap();
                let last_digit = digits.last().unwrap_or(first_digit);
                10 * first_digit + last_digit
            })
            .sum()
    }

    fn solve_part2(input: &str) -> u32 {
        input
            .trim()
            .lines()
            .map(|line| {
                let chars: Vec<char> = line.chars().collect();
                let mut digits = (0..(chars.len())).filter_map(|i| is_valid_digit(&chars, i));
                let first_digit = digits.next().unwrap();
                let last_digit = digits.last().unwrap_or(first_digit);
                10 * first_digit + last_digit
            })
            .sum()
    }
}
const LETTERS_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn is_valid_digit(chars: &[char], i: usize) -> Option<u32> {
    if let Some(digit) = chars[i].to_digit(10) {
        return Some(digit);
    }
    let n_chars_lefts = chars.len() - i;

    // try spellout letters
    LETTERS_DIGITS
        .into_iter()
        .enumerate()
        .find_map(|(idx, letters_digit)| {
            let n_letters = letters_digit.len();
            (n_chars_lefts >= n_letters)
                .then(|| {
                    letters_digit
                        .chars()
                        .zip(chars[i..(i + n_letters)].iter())
                        .all(|(a, &b)| a == b)
                        .then_some((idx + 1) as u32)
                })
                .flatten()
        })
}
