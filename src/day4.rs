use std::collections::HashSet;

use crate::Puzzle;

struct Card {
    winners: HashSet<usize>,
    numbers: HashSet<usize>,
}

impl Card {
    pub fn parse(input: &str) -> Self {
        let (winners, numbers) = input.split_once('|').unwrap();
        let (_, winners) = winners.split_once(':').unwrap();
        let winners = winners
            .split_whitespace()
            .map(|part| part.parse::<usize>().unwrap())
            .collect();
        let numbers = numbers
            .split_whitespace()
            .map(|part| part.parse::<usize>().unwrap())
            .collect();

        Self { winners, numbers }
    }

    pub fn wins(&self) -> usize {
        self.numbers
            .iter()
            .filter(|num| self.winners.contains(*num))
            .count()
    }

    pub fn score(&self) -> usize {
        let count = self.wins();
        if count > 0 {
            2_usize.pow(count as u32 - 1)
        } else {
            0
        }
    }
}

pub struct Day4;

impl Puzzle for Day4 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        input.lines().map(|line| Card::parse(line).score()).sum()
    }

    fn part2(input: &str) -> Self::Output {
        let cards: Vec<Card> = input.lines().map(Card::parse).collect();
        let mut result = vec![1; cards.len()];

        for (idx, card) in cards.iter().enumerate() {
            for i in 1..=card.wins().min(cards.len() - 1) {
                result[idx + i] += result[idx];
            }
        }

        result.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day4;
    use crate::Puzzle;

    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn example1() {
        assert_eq!(Day4::part1(INPUT), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(Day4::part2(INPUT), 30);
    }
}
