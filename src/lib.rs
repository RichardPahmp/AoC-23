pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod slurp;

use std::{fmt::Display, time::Instant};

pub trait Puzzle {
    type Output: Display;

    fn part1(input: &str) -> Self::Output;
    fn part2(input: &str) -> Self::Output;
}

pub fn run<T: Puzzle>(day: u8, input: &str) {
    println!("Day {}:", day);

    let start1 = Instant::now();
    let answer1 = T::part1(input);
    let time1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = T::part2(input);
    let time2 = start2.elapsed();

    let total = time1 + time2;

    println!("  Part 1: {} - {}s", answer1, time1.as_secs_f64());
    println!("  Part 2: {} - {}s", answer2, time2.as_secs_f64());
    println!("  Total: {}s", total.as_secs_f64());
    println!();
}
