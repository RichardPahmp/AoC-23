use std::fmt::Debug;

use crate::{
    slurp::{
        self, delimited, map, map_res, nl, opt, pair, separated_list, separated_pair, tag,
        take_while1, tuple, Res,
    },
    Puzzle,
};

#[derive(PartialEq, Eq)]
struct Range {
    source: usize,
    destination: usize,
    len: usize,
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}..{} -> {}..{}",
            self.source,
            self.source + self.len,
            self.destination,
            self.destination + self.len
        )
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.source.cmp(&other.source)
    }
}

impl Range {
    pub fn new(destination: usize, source: usize, len: usize) -> Self {
        Self {
            destination,
            source,
            len,
        }
    }

    pub fn contains(&self, value: usize) -> bool {
        value >= self.source && value < self.source + self.len
    }

    pub fn map(&self, value: usize) -> Option<usize> {
        if self.contains(value) {
            let offset = value - self.source;
            Some(self.destination + offset)
        } else {
            None
        }
    }

    pub fn invert(&mut self) {
        std::mem::swap(&mut self.source, &mut self.destination);
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    pub fn new(mut ranges: Vec<Range>) -> Self {
        ranges.sort();
        Self { ranges }
    }

    pub fn invert(&mut self) {
        self.ranges.iter_mut().for_each(Range::invert);
        self.ranges.sort();
    }

    pub fn map(&self, value: usize) -> usize {
        for range in &self.ranges {
            if let Some(v) = range.map(value) {
                return v;
            }
        }
        value
    }
}

fn num(input: &str) -> slurp::Res<&str, usize> {
    map_res(
        take_while1(|ch: char| ch.is_ascii_digit()),
        str::parse::<usize>,
    )(input)
}

fn ws1(input: &str) -> slurp::Res<&str, ()> {
    take_while1(char::is_whitespace)(input).map(|(rem, _)| (rem, ()))
}

fn parse_range(input: &str) -> slurp::Res<&str, Range> {
    let (rem, range) = map(tuple((num, delimited(ws1, num), num)), |(a, b, c)| {
        Range::new(a, b, c)
    })(input)?;
    Ok((rem, range))
}

fn parse_map<'a>(input: &'a str, title: &str) -> Res<&'a str, Map> {
    let (i, _) = opt(nl())(input)?;
    let (i, _) = pair(tag(title), nl())(i)?;
    let (i, ranges) = separated_list(parse_range, nl())(i)?;
    Ok((i, Map::new(ranges)))
}

fn parse_seeds(input: &str) -> Res<&str, Vec<usize>> {
    let (i, (_, seeds, _)) = tuple(("seeds: ", separated_list(num, ' '), nl()))(input)?;
    Ok((i, seeds))
}

fn parse_seeds2(input: &str) -> Res<&str, Vec<(usize, usize)>> {
    let (i, (_, seeds, _)) = tuple((
        tag("seeds: "),
        separated_list(separated_pair(num, ' ', num), ' '),
        nl(),
    ))(input)?;
    Ok((i, seeds))
}

fn parse_maps(input: &str) -> Res<&str, Vec<Map>> {
    let (i, seedsoil) = parse_map(input, "seed-to-soil map:")?;
    let (i, soilfertilizer) = parse_map(i, "soil-to-fertilizer map:")?;
    let (i, fertilizerwater) = parse_map(i, "fertilizer-to-water map:")?;
    let (i, waterlight) = parse_map(i, "water-to-light map:")?;
    let (i, lighttemperature) = parse_map(i, "light-to-temperature map:")?;
    let (i, temperaturehumidity) = parse_map(i, "temperature-to-humidity map:")?;
    let (i, humiditylocation) = parse_map(i, "humidity-to-location map:")?;

    Ok((
        i,
        vec![
            seedsoil,
            soilfertilizer,
            fertilizerwater,
            waterlight,
            lighttemperature,
            temperaturehumidity,
            humiditylocation,
        ],
    ))
}

fn parse_input(input: &str) -> Res<&str, (Vec<usize>, Vec<Map>)> {
    let (i, seeds) = parse_seeds(input)?;
    let (i, maps) = parse_maps(i)?;

    Ok((i, (seeds, maps)))
}

#[allow(clippy::type_complexity)]
fn parse_input2(input: &str) -> Res<&str, (Vec<(usize, usize)>, Vec<Map>)> {
    let (i, seeds) = parse_seeds2(input)?;
    let (i, maps) = parse_maps(i)?;

    Ok((i, (seeds, maps)))
}

pub struct Day5;

impl Puzzle for Day5 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let (_, (seeds, maps)) = parse_input(input).unwrap();
        let min = seeds
            .into_iter()
            .map(|seed| {
                let location = maps.iter().fold(seed, |value, map| map.map(value));
                location
            })
            .min()
            .unwrap();

        min
    }

    fn part2(input: &str) -> Self::Output {
        let (_, (seeds, mut maps)) = parse_input2(input).unwrap();
        let seeds = seeds
            .into_iter()
            .map(|(start, end)| start..start + end)
            .collect::<Vec<_>>();

        maps.reverse();
        maps.iter_mut().for_each(Map::invert);

        for loc in 0..10000000000 {
            let seed = maps.iter().fold(loc, |value, map| map.map(value));
            for range in &seeds {
                if range.contains(&seed) {
                    return loc;
                }
            }
        }
        panic!("No seed found.");
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_seeds2, Day5};
    use crate::Puzzle;

    const INPUT: &str = include_str!("input/day5ex");
    const INPUT2: &str = include_str!("input/day5");

    #[test]
    fn example1() {
        assert_eq!(Day5::part1(INPUT), 35);
    }

    #[test]
    fn example2() {
        assert_eq!(Day5::part2(INPUT), 46);
    }

    #[ignore]
    #[test]
    fn solution2() {
        assert_eq!(Day5::part2(INPUT2), 59370572);
    }

    #[test]
    fn ranges() {
        let (_, mut ranges) = parse_seeds2(INPUT2).unwrap();
        ranges.sort();
        let mut prev = 0;
        for (start, end) in ranges {
            println!("{}-{} = {}", start, end, start + end);
            println!("{}", prev > start);
            prev = end;
        }
    }
}
