use crate::Puzzle;

fn calibrate(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut iter = line.chars().filter_map(|ch| ch.to_digit(10));

            let first = iter
                .next()
                .expect("Line should contain at least one digit.");
            let second = iter.last().unwrap_or(first);
            first * 10 + second
        })
        .sum()
}

fn starts_with_digit(v: &str) -> Option<u32> {
    Some(match v {
        v if v.starts_with("one") => 1,
        v if v.starts_with("two") => 2,
        v if v.starts_with("three") => 3,
        v if v.starts_with("four") => 4,
        v if v.starts_with("five") => 5,
        v if v.starts_with("six") => 6,
        v if v.starts_with("seven") => 7,
        v if v.starts_with("eight") => 8,
        v if v.starts_with("nine") => 9,
        _ => return v.chars().next().and_then(|ch| ch.to_digit(10)),
    })
}

struct DigitIterator<'a> {
    line: &'a str,
}

impl<'a> Iterator for DigitIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.line.is_empty() {
            let digit = starts_with_digit(self.line);
            self.line = &self.line[1..];
            if digit.is_some() {
                return digit;
            }
        }
        None
    }
}

fn calibrate2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut iter = DigitIterator { line };
            let first = iter.next().unwrap();
            let last = iter.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

pub struct Day1;

impl Puzzle for Day1 {
    type Output = u32;

    fn part1(input: &str) -> Self::Output {
        calibrate(input)
    }

    fn part2(input: &str) -> Self::Output {
        calibrate2(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::day1::Day1;
    use crate::Puzzle;

    #[test]
    fn example1() {
        let input = r#"1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"#;

        assert_eq!(Day1::part1(input), 142)
    }

    #[test]
    fn example2() {
        let input = r#"two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"#;

        assert_eq!(Day1::part2(input), 281)
    }
}
