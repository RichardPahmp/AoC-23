use std::{cmp::Ordering, str::FromStr};

use crate::{
    slurp::{chr, map, separated_pair, tuple, ParseError, Res},
    Puzzle,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    pub fn from_char(ch: char) -> Self {
        match ch {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::N9,
            '8' => Card::N8,
            '7' => Card::N7,
            '6' => Card::N6,
            '5' => Card::N5,
            '4' => Card::N4,
            '3' => Card::N3,
            '2' => Card::N2,
            _ => panic!("Invalid card: {}", ch),
        }
    }

    pub fn from_char2(ch: char) -> Self {
        let card = Self::from_char(ch);
        if card == Card::J {
            Card::Joker
        } else {
            card
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

fn get_kind(mut cards: [Card; 5]) -> HandKind {
    cards.sort();
    cards.reverse();

    let mut max = 0;
    let mut pen = 0;
    let mut count = 1;
    let mut current = cards[0];

    for card in cards.into_iter().skip(1) {
        if current == card {
            count += 1;
        } else {
            count = 1;
            current = card;
        }

        if count > max {
            max = count;
        } else {
            pen = pen.max(count);
        }
    }

    match (max, pen) {
        (5, _) => HandKind::FiveKind,
        (4, _) => HandKind::FourKind,
        (3, 2) => HandKind::FullHouse,
        (3, _) => HandKind::ThreeKind,
        (2, 2) => HandKind::TwoPair,
        (2, _) => HandKind::OnePair,
        _ => HandKind::HighCard,
    }
}

fn get_kind2(mut cards: [Card; 5]) -> HandKind {
    cards.sort();
    cards.reverse();

    let mut max = 1;
    let mut pen = 1;
    let mut count = 1;
    let mut current = cards[0];
    let mut jokers = if current == Card::Joker { 1 } else { 0 };

    for card in cards.into_iter().skip(1) {
        if card == Card::Joker {
            jokers += 1;
            continue;
        } else if current == card {
            count += 1;
        } else {
            count = 1;
            current = card;
        }

        if count > max {
            max = count;
        } else {
            pen = pen.max(count);
        }
    }

    max = (max + jokers).min(5);

    match (max, pen) {
        (5, _) => HandKind::FiveKind,
        (4, _) => HandKind::FourKind,
        (3, 2) => HandKind::FullHouse,
        (3, _) => HandKind::ThreeKind,
        (2, 2) => HandKind::TwoPair,
        (2, _) => HandKind::OnePair,
        _ => HandKind::HighCard,
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    kind: HandKind,
    bid: usize,
}

impl Hand {
    pub fn new(cards: [Card; 5], bid: usize) -> Self {
        Self {
            cards,
            kind: get_kind(cards),
            bid,
        }
    }

    pub fn new2(cards: [Card; 5], bid: usize) -> Self {
        Self {
            cards,
            kind: get_kind2(cards),
            bid,
        }
    }
}

fn parse<T: FromStr>(input: &str) -> Res<&str, T> {
    let t = input.parse::<T>().map_err(|_| ParseError::MapError)?;
    Ok((&"", t))
}

fn card<'a>() -> impl FnMut(&'a str) -> Res<&'a str, Card> {
    map(chr(), Card::from_char)
}

fn card2<'a>() -> impl FnMut(&'a str) -> Res<&'a str, Card> {
    map(chr(), Card::from_char2)
}

fn parse_cards(input: &str) -> Res<&str, [Card; 5]> {
    let (i, (a, b, c, d, e)) = tuple((card(), card(), card(), card(), card()))(input).unwrap();
    Ok((i, [a, b, c, d, e]))
}

fn parse_cards2(input: &str) -> Res<&str, [Card; 5]> {
    let (i, (a, b, c, d, e)) = tuple((card2(), card2(), card2(), card2(), card2()))(input).unwrap();
    Ok((i, [a, b, c, d, e]))
}

fn parse_hand(input: &str) -> ([Card; 5], usize) {
    let (_, (cards, bid)) = separated_pair(parse_cards, ' ', parse::<usize>)(input).unwrap();
    (cards, bid)
}

fn parse_hand2(input: &str) -> ([Card; 5], usize) {
    let (_, (cards, bid)) = separated_pair(parse_cards2, ' ', parse::<usize>)(input).unwrap();
    (cards, bid)
}

pub struct Day7;

impl Puzzle for Day7 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let mut hands: Vec<Hand> = input
            .lines()
            .map(parse_hand)
            .map(|(cards, bid)| Hand::new(cards, bid))
            .collect();
        hands.sort_by(|left, right| {
            let ordering = left.kind.cmp(&right.kind);
            if let Ordering::Equal = ordering {
                left.cards.cmp(&right.cards)
            } else {
                ordering
            }
        });
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i + 1))
            .sum()
    }

    fn part2(input: &str) -> Self::Output {
        let mut hands: Vec<Hand> = input
            .lines()
            .map(parse_hand2)
            .map(|(cards, bid)| Hand::new2(cards, bid))
            .collect();
        hands.sort_by(|left, right| {
            let ordering = left.kind.cmp(&right.kind);
            if let Ordering::Equal = ordering {
                left.cards.cmp(&right.cards)
            } else {
                ordering
            }
        });
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i + 1))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_hand, Card, Day7, Hand, HandKind};
    use crate::{day7::get_kind2, Puzzle};

    const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn parse1() {
        let (cards, bid) = parse_hand("K4444 123");
        let hand = Hand::new(cards, bid);
        assert_eq!(
            [Card::K, Card::N4, Card::N4, Card::N4, Card::N4],
            hand.cards
        );
        assert_eq!(123, hand.bid);
        assert_eq!(HandKind::FourKind, hand.kind);
    }

    #[test]
    fn example1() {
        assert_eq!(Day7::part1(INPUT), 6440)
    }

    #[test]
    fn example2() {
        assert_eq!(Day7::part2(INPUT), 5905)
    }

    #[test]
    fn example3() {
        const INPUT2: &str = r#"2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41"#;
        assert_eq!(Day7::part2(INPUT2), 6839);
    }

    #[test]
    fn jokers() {
        assert_eq!(
            get_kind2([Card::N2, Card::Joker, Card::Joker, Card::Joker, Card::Joker]),
            HandKind::FiveKind
        );
    }

    #[test]
    fn order() {
        assert!(Card::A > Card::K);
        assert!(Card::A > Card::N2);
        assert!(Card::N3 > Card::N2);
        assert!(Card::Joker < Card::N2);
    }
}
