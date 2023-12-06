use crate::{
    slurp::{
        self, delimited, map, map_res, opt, pair, separated_list, separated_pair, tag, take_while,
        take_while1, tuple,
    },
    Puzzle,
};

#[derive(Debug)]
struct Range {
    destination: usize,
    source: usize,
    len: usize,
}

impl Range {
    pub fn new(destination: usize, source: usize, len: usize) -> Self {
        Self {
            destination,
            source,
            len,
        }
    }

    pub fn map(&self, value: usize) -> Option<usize> {
        if value >= self.source && value <= self.source + self.len {
            let offset = value - self.source;
            Some(self.destination + offset)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
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

fn ws(input: &str) -> slurp::Res<&str, ()> {
    take_while(char::is_whitespace)(input).map(|(rem, _)| (rem, ()))
}

fn nl(input: &str) -> slurp::Res<&str, ()> {
    let (rem, _) = pair(opt(tag("\r")), tag("\n"))(input)?;
    Ok((rem, ()))
}

fn parse_range(input: &str) -> slurp::Res<&str, Range> {
    println!("parse_range: {:?}", input);
    let (rem, range) = map(tuple((num, delimited(ws1, num), num)), |(a, b, c)| {
        Range::new(a, b, c)
    })(input)?;
    Ok((rem, range))
}

fn parse_map<'a>(input: &'a str, title: &str) -> slurp::Res<&'a str, Map> {
    println!("parse_map: {:?}", input);
    let (rem, (_, _, ranges)) = tuple((tag(title), nl, separated_list(parse_range, nl)))(input)?;
    Ok((rem, Map { ranges }))
}

fn parse_input(input: &str) -> Option<(Vec<usize>, Vec<Map>)> {
    let mut chunks = input.split("\r\n\r\n");
    let (_, (_, seeds)) =
        pair(tag("seeds: "), separated_list(num, tag(" ")))(chunks.next()?).unwrap();

    let (_, seedsoil) = parse_map(chunks.next()?, "seed-to-soil map:").unwrap();
    let (_, soilfertilizer) = parse_map(chunks.next()?, "soil-to-fertilizer map:").unwrap();
    let (_, fertilizerwater) = parse_map(chunks.next()?, "fertilizer-to-water map:").unwrap();
    let (_, waterlight) = parse_map(chunks.next()?, "water-to-light map:").unwrap();
    let (_, lighttemperature) = parse_map(chunks.next()?, "light-to-temperature map:").unwrap();
    let (_, temperaturehumidity) =
        parse_map(chunks.next()?, "temperature-to-humidity map:").unwrap();
    let (_, humiditylocation) = parse_map(chunks.next()?, "humidity-to-location map:").unwrap();
    assert!(chunks.next().is_none());
    Some((
        seeds,
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

fn parse_input2(input: &str) -> Option<(Vec<(usize, usize)>, Vec<Map>)> {
    let mut chunks = input.split("\r\n\r\n");
    let (_, (_, seeds)) = pair(
        tag("seeds: "),
        separated_list(separated_pair(num, tag(" "), num), tag(" ")),
    )(chunks.next()?)
    .unwrap();

    let (_, seedsoil) = parse_map(chunks.next()?, "seed-to-soil map:").unwrap();
    let (_, soilfertilizer) = parse_map(chunks.next()?, "soil-to-fertilizer map:").unwrap();
    let (_, fertilizerwater) = parse_map(chunks.next()?, "fertilizer-to-water map:").unwrap();
    let (_, waterlight) = parse_map(chunks.next()?, "water-to-light map:").unwrap();
    let (_, lighttemperature) = parse_map(chunks.next()?, "light-to-temperature map:").unwrap();
    let (_, temperaturehumidity) =
        parse_map(chunks.next()?, "temperature-to-humidity map:").unwrap();
    let (_, humiditylocation) = parse_map(chunks.next()?, "humidity-to-location map:").unwrap();
    assert!(chunks.next().is_none());
    Some((
        seeds,
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

pub struct Day5;

impl Puzzle for Day5 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let (seeds, maps) = parse_input(input).unwrap();
        let min = seeds
            .iter()
            .map(|seed| {
                let location = maps.iter().fold(*seed, |value, map| map.map(value));
                location
            })
            .min()
            .unwrap();

        min
    }

    fn part2(input: &str) -> Self::Output {
        let (seeds, maps) = parse_input2(input).unwrap();
        let seeds: Vec<usize> = seeds.into_iter().flat_map(|(a, b)| a..=a + b).collect();
        dbg!(&seeds);
        let min = seeds
            .iter()
            .map(|seed| {
                let location = maps.iter().fold(*seed, |value, map| map.map(value));
                location
            })
            .min()
            .unwrap();

        min
    }
}

#[cfg(test)]
mod tests {
    use super::Day5;
    use crate::Puzzle;

    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn example1() {
        assert_eq!(Day5::part1(INPUT), 35);
    }

    #[test]
    fn example2() {
        assert_eq!(Day5::part2(INPUT), 46);
    }
}
