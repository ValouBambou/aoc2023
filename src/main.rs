mod array2d;
mod math_utils;
mod parse_utils;
mod rect2d;
mod vec2d;

use aoc_macros::{days_modules, match_test_days, match_time_days};
use std::{
    env::{args, Args},
    fmt::{Debug, Display},
    num::ParseIntError,
    time::{Duration, SystemTime},
};

days_modules!();

fn main() -> Result<(), ErrorCLI> {
    type A = ExpectedArgsCLI;
    let cli: A = args().try_into()?;
    match cli {
        A::TimeAllDays => {
            let mut total_time = Duration::ZERO;
            for n in 1..=25 {
                total_time += match_time_days!();
            }
            println!("Total time = {total_time:?}");
        }
        A::ExecDay { number: n, timed } => {
            if timed {
                match_time_days!();
            } else {
                match_test_days!();
            }
        }
    }
    Ok(())
}

pub trait DaySolution<T: Debug + Display + Eq = usize> {
    const DAY: u8;
    const INPUT: &'static str;
    const EXAMPLE_1: &'static str;
    const EXAMPLE_2: &'static str;
    const EXAMPLE_ANSWER_1: T;
    const EXAMPLE_ANSWER_2: T;

    fn solve_part1(input: &str) -> T;
    fn solve_part2(input: &str) -> T;
    fn test() {
        let n = Self::DAY;
        let ex1 = Self::solve_part1(Self::EXAMPLE_1);
        let ex1_ans = Self::EXAMPLE_ANSWER_1;
        assert_eq!(
            ex1, ex1_ans,
            "Day {n} part 1 failed expected {ex1_ans}, got {ex1}",
        );

        println!("Day {n} part 1 Example ✅");
        let ans1 = Self::solve_part1(Self::INPUT);
        println!("Day {n} part 1 Solution is {ans1}");

        // same for part 2
        let ex2 = Self::solve_part2(Self::EXAMPLE_2);
        let ex2_ans = Self::EXAMPLE_ANSWER_2;
        assert_eq!(
            ex2, ex2_ans,
            "Day {n} part 2 failed expected {ex2_ans}, got {ex2}",
        );
        println!("Day {n} part 2 Example ✅");
        let ans2 = Self::solve_part2(Self::INPUT);
        println!("Day {n} part 2 Solution is {ans2}");
    }
    fn time() -> Duration {
        let n = Self::DAY;
        let start1 = SystemTime::now();
        let ans1 = Self::solve_part1(Self::INPUT);
        let duration1 = start1.elapsed().unwrap();
        println!("Day {n} part 1 Solution is {ans1} took {duration1:?}");
        let start2 = SystemTime::now();
        let ans2 = Self::solve_part2(Self::INPUT);
        let duration2 = start2.elapsed().unwrap();
        println!("Day {n} part 2 Solution is {ans2} took {duration2:?}");
        duration1 + duration2
    }
}
enum ExpectedArgsCLI {
    TimeAllDays,
    ExecDay { number: u8, timed: bool },
}

impl Debug for ErrorCLI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        type E = ErrorCLI;
        match self {
            E::NotIntegerDayNum(n) => {
                write!(f, "First argument should be a day number not {n:?}")
            }
            E::DayNumOutOfRange(n) => {
                write!(f, "Day number {n} is not in the expected range 1..=25")
            }
            E::InvalidFlag(flag) => {
                write!(
                    f,
                    "Invalid second argument {flag:?}, expected flag --time or --no-time."
                )
            }
            E::TooMuchArguments(args) => {
                write!(f, "Too much arguments given {args:?}\n Usage 1, time everything: cargo run \n | Usage 2, test one day: cargo run -- <day_n> [--time]")
            }
        }
    }
}

enum ErrorCLI {
    NotIntegerDayNum(ParseIntError),
    InvalidFlag(String),
    DayNumOutOfRange(u8),
    TooMuchArguments(Args),
}

impl From<ParseIntError> for ErrorCLI {
    fn from(value: ParseIntError) -> Self {
        Self::NotIntegerDayNum(value)
    }
}

impl TryFrom<Args> for ExpectedArgsCLI {
    type Error = ErrorCLI;
    fn try_from(value: Args) -> Result<Self, Self::Error> {
        if value.len() == 1 {
            return Ok(Self::TimeAllDays);
        }
        if value.len() > 3 {
            return Err(ErrorCLI::TooMuchArguments(value));
        }
        let mut args = value.skip(1);
        // 2 arguments max
        let number: u8 = args.next().unwrap().parse()?;
        if !(1..=25).contains(&number) {
            return Err(ErrorCLI::DayNumOutOfRange(number));
        }

        let flag = args.next().unwrap_or("--no-time".to_owned());
        let timed = match flag.as_str() {
            "--no-time" => false,
            "--time" => true,
            _ => {
                return Err(ErrorCLI::InvalidFlag(flag));
            }
        };
        Ok(Self::ExecDay { number, timed })
    }
}
