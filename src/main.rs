use aoc23::{
    day1::Day1, day2::Day2, day3::Day3, day4::Day4, day5::Day5, day6::Day6, day7::Day7, day8::Day8,
    day9::Day9, run,
};
use clap::Parser;

#[derive(Parser)]
struct Args {
    day: Option<usize>,
}

fn main() {
    let _args = Args::parse();

    run::<Day1>(1, include_str!("input/day1"));
    run::<Day2>(2, include_str!("input/day2"));
    run::<Day3>(3, include_str!("input/day3"));
    run::<Day4>(4, include_str!("input/day4"));
    run::<Day5>(5, include_str!("input/day5"));
    run::<Day6>(6, include_str!("input/day6"));
    run::<Day7>(7, include_str!("input/day7"));
    run::<Day8>(8, include_str!("input/day8"));
    run::<Day9>(9, include_str!("input/day9"));
}
