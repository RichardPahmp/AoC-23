use std::collections::HashMap;

use crate::{
    slurp::{opt, pair, separated_list, separated_pair, take_while1, tuple, Res},
    Puzzle,
};

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
struct Label([u8; 3]);

impl Label {
    pub fn new(label: &[u8]) -> Self {
        Self(label.try_into().expect("Slice is 3 bytes long."))
    }

    pub fn is_start(&self) -> bool {
        self.0[2] == b'A'
    }

    pub fn is_end(&self) -> bool {
        self.0[2] == b'Z'
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    left: Label,
    right: Label,
}

impl Node {
    pub fn new(left: &[u8], right: &[u8]) -> Self {
        Self {
            left: Label::new(left),
            right: Label::new(right),
        }
    }
}

fn element(input: &[u8]) -> Res<&[u8], &[u8]> {
    take_while1(|b: u8| b.is_ascii_alphanumeric())(input)
}

fn newline(input: &[u8]) -> Res<&[u8], (Option<u8>, u8)> {
    pair(opt(b'\r'), b'\n')(input)
}

type Network = HashMap<Label, Node>;

fn parse_network(input: &[u8]) -> Res<&[u8], Network> {
    let parse_line = tuple((
        element,
        &b" = ("[..],
        separated_pair(element, &b", "[..], element),
        b')',
    ));
    let (i, map) = separated_list(parse_line, newline)(input)?;
    let network = map
        .iter()
        .map(|(key, _, (left, right), _)| (Label::new(key), Node::new(left, right)))
        .collect::<HashMap<Label, Node>>();
    Ok((i, network))
}

fn cycle_count(network: &Network, start: Label, route: &[u8]) -> usize {
    let mut loc = start;
    let mut count = 0;
    while !loc.is_end() {
        for ch in route {
            let Node { left, right } = network[&loc];
            let next = match ch {
                b'L' => left,
                b'R' => right,
                _ => panic!("Invalid character in route."),
            };
            loc = next;
        }
        count += 1;
        if loc.is_end() {
            return count;
        }
    }
    unreachable!();
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != 0 {
        let tmp = a;
        a = b % a;
        b = tmp;
    }
    b
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

pub struct Day8;

impl Puzzle for Day8 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let input = input.as_bytes();
        let (i, route) = take_while1(|b: u8| b.is_ascii_alphabetic())(input).unwrap();
        let (i, _) = pair(newline, newline)(i).unwrap();
        let (_, network) = parse_network(i).unwrap();

        let mut i = 0;
        let mut current = Label::new(&b"AAA"[..]);
        loop {
            for ch in route {
                let Node { left, right } = network[&current];
                let next = match ch {
                    b'L' => left,
                    b'R' => right,
                    _ => panic!("Invalid character in route."),
                };
                current = next;
                i += 1;
                if current.is_end() {
                    return i;
                }
            }
        }
    }

    fn part2(input: &str) -> Self::Output {
        let input = input.as_bytes();
        let (i, route) = take_while1(|b: u8| b.is_ascii_alphabetic())(input).unwrap();
        let (i, _) = pair(newline, newline)(i).unwrap();
        let (i, network) = parse_network(i).unwrap();
        assert_eq!(i, &[]);

        let ghosts = network
            .keys()
            .filter_map(|&key| key.is_start().then_some(key))
            .collect::<Vec<_>>();

        let lcm = ghosts
            .into_iter()
            .map(|ghost| cycle_count(&network, ghost, route))
            .reduce(lcm)
            .unwrap();

        lcm * route.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{slurp::take_while1, Puzzle};

    const EXAMPLE1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    const EXAMPLE2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn example1() {
        assert_eq!(Day8::part1(EXAMPLE1), 2)
    }

    #[test]
    fn example2() {
        assert_eq!(Day8::part2(EXAMPLE2), 6)
    }

    #[test]
    fn test_parse() {
        println!("{:?}", EXAMPLE2);
        let i = EXAMPLE2.as_bytes();
        println!("{:?}", i);
        let (i, item) = take_while1(|b: u8| b.is_ascii_alphanumeric())(i).unwrap();
        dbg!(item);
        assert_eq!(item, b"LR");
        let (i, item) = newline(i).unwrap();
        dbg!(item);
        assert_eq!(item, (None, b'\n'));
        let (i, item) = newline(i).unwrap();
        dbg!(item);
        assert_eq!(item, (None, b'\n'));
        assert_eq!(i[0..=2], b"11A"[..], "{}", std::ascii::escape_default(i[0]));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(35, 21), 7)
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(7, 11), 77)
    }
}
