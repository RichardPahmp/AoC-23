use crate::{
    slurp::{self, map_res, opt, pair, separated_list, tag, take_while1, Res},
    Puzzle,
};

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    pub fn new(time: usize, distance: usize) -> Self {
        Self { time, distance }
    }
}

fn num(input: &str) -> slurp::Res<&str, usize> {
    map_res(
        take_while1(|ch: char| ch.is_ascii_digit()),
        str::parse::<usize>,
    )(input)
}

fn whitespace(input: &str) -> Res<&str, &str> {
    take_while1(|ch| ch == ' ')(input)
}

fn nl(input: &str) -> slurp::Res<&str, ()> {
    let (rem, _) = pair(opt(tag("\r")), tag("\n"))(input)?;
    Ok((rem, ()))
}

fn parse_input(input: &str) -> Vec<Race> {
    let (i, _) = pair(tag("Time:"), whitespace)(input).unwrap();
    let (i, times) = separated_list(num, whitespace)(i).unwrap();
    let (i, _) = nl(i).unwrap();
    let (i, _) = pair(tag("Distance:"), whitespace)(i).unwrap();
    let (_, distances) = separated_list(num, whitespace)(i).unwrap();

    times
        .into_iter()
        .zip(distances)
        .map(|(time, dist)| Race::new(time, dist))
        .collect()
}

fn parse_input2(input: &str) -> Race {
    let (i, _) = pair(tag("Time:"), whitespace)(input).unwrap();
    let (i, times) = take_while1(|ch| ch != '\n')(i).unwrap();
    let time: String = times.chars().filter(char::is_ascii_digit).collect();
    let (i, _) = nl(i).unwrap();
    let (i, _) = pair(tag("Distance:"), whitespace)(i).unwrap();
    let (_, distances) = take_while1(|ch| ch != '\n')(i).unwrap();
    let distance: String = distances.chars().filter(char::is_ascii_digit).collect();
    Race::new(
        time.parse::<usize>().unwrap(),
        distance.parse::<usize>().unwrap(),
    )
}

fn roots(race: &Race) -> (f64, f64) {
    let t = race.time as f64;
    let d = race.distance as f64;
    let base = t / 2.0;
    let root = ((t / 2.0).powf(2.0) - d).sqrt();
    let a = base - root;
    let b = base + root;
    (a.min(b), a.max(b))
}

fn winning_times(race: &Race) -> usize {
    let (low, high) = roots(race);
    let low = low.floor() as usize + 1;
    let high = high.ceil();
    let high = high as usize;
    let high = high - 1;
    let range = low..=high;
    range.count()
}

pub struct Day6;

impl Puzzle for Day6 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let races = parse_input(input);
        races.iter().map(winning_times).product()
    }

    fn part2(input: &str) -> Self::Output {
        let race = parse_input2(input);
        winning_times(&race)
    }
}

#[cfg(test)]
mod tests {
    use super::Day6;
    use crate::Puzzle;

    const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn example1() {
        assert_eq!(Day6::part1(INPUT), 288);
    }

    #[test]
    fn example2() {
        assert_eq!(Day6::part2(INPUT), 71503);
    }

    fn printfloat(f: f32) {
        println!("{}", f);
    }

    #[test]
    #[ignore]
    fn what() {
        let f: f32 = 44875631.0;
        println!("{}", f);
        println!("{:?}", f);
        dbg!(&f);
        printfloat(f);
        let u: usize = unsafe { f.to_int_unchecked() };
        assert_eq!(u, 44875630);
    }
}
