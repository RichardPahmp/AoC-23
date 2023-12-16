use std::fmt::Debug;

use crate::{
    slurp::{map_res, separated_list, take_while1, Res},
    Puzzle,
};

struct Sequence {
    history: Vec<isize>,
}

impl Sequence {
    pub fn new(history: impl Into<Vec<isize>>) -> Self {
        let history = history.into();
        assert!(!history.is_empty());
        Self { history }
    }

    pub fn from_str(input: &str) -> Self {
        fn num(input: &str) -> Res<&str, isize> {
            map_res(
                take_while1(|ch: char| !ch.is_ascii_whitespace()),
                |i: &str| i.parse::<isize>(),
            )(input)
        }

        let (_, nums) = separated_list(num, ' ')(input).unwrap();
        Self::new(nums)
    }

    pub fn first(&self) -> isize {
        *self.history.first().unwrap()
    }

    pub fn last(&self) -> isize {
        *self.history.last().unwrap()
    }

    pub fn differences(&self) -> Sequence {
        Self::new(
            self.history
                .windows(2)
                .map(|win| win[1] - win[0])
                .collect::<Vec<_>>(),
        )
    }

    pub fn zero(&self) -> bool {
        self.history.iter().all(|&i| i == 0)
    }

    pub fn extrapolate(&self) -> isize {
        let diff = self.differences();
        if diff.zero() {
            self.last()
        } else {
            self.last() + diff.extrapolate()
        }
    }

    pub fn extrapolate_back(&self) -> isize {
        let diff = self.differences();
        if diff.zero() {
            self.first()
        } else {
            self.first() - diff.extrapolate_back()
        }
    }
}

impl Debug for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for n in &self.history {
            f.write_fmt(format_args!("{} ", *n))?;
        }
        Ok(())
    }
}

pub struct Day9;

impl Puzzle for Day9 {
    type Output = isize;

    fn part1(input: &str) -> Self::Output {
        let sequences = input.lines().map(Sequence::from_str).collect::<Vec<_>>();
        sequences.iter().map(|seq| seq.extrapolate()).sum()
    }

    fn part2(input: &str) -> Self::Output {
        let sequences = input.lines().map(Sequence::from_str).collect::<Vec<_>>();
        sequences.iter().map(|seq| seq.extrapolate_back()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    pub fn example1() {
        assert_eq!(Day9::part1(EXAMPLE), 114);
    }

    #[test]
    pub fn example2() {
        assert_eq!(Day9::part2(EXAMPLE), 2);
    }
}
