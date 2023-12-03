use crate::Puzzle;

#[derive(Debug, Default, Clone, Copy)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn can_contain(&self, other: &Bag) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn combine(&mut self, other: Bag) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}

#[derive(Debug)]
pub struct Game {
    number: u32,
    pulls: Vec<Bag>,
}

impl Game {
    fn minimum_bag(&self) -> Bag {
        self.pulls
            .iter()
            .fold(Bag::default(), |mut bag, other| bag.combine(*other))
    }
}

fn tag<'a>(tag: &str, input: &'a str) -> Option<&'a str> {
    let expected = input.get(..tag.len())?;
    (tag == expected).then(|| &input[tag.len()..])
}

fn parse_game(input: &str) -> Game {
    let (prefix, game) = input.split_once(':').unwrap();
    let number = tag("Game ", prefix).unwrap().parse::<u32>().unwrap();
    let mut pulls = Vec::new();
    for pull in game.split(';') {
        let mut bag = Bag::default();
        for pair in pull.trim().split(", ") {
            let (count, color) = pair.split_once(' ').unwrap();
            let count = count.parse::<u32>().unwrap();
            *match color {
                "red" => &mut bag.red,
                "green" => &mut bag.green,
                "blue" => &mut bag.blue,
                _ => panic!(),
            } = count;
        }
        pulls.push(bag);
    }

    Game { number, pulls }
}

pub struct Day2;

impl Puzzle for Day2 {
    type Output = u32;

    fn part1(input: &str) -> Self::Output {
        let reference = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };

        let games: Vec<Game> = input.lines().map(parse_game).collect();
        games
            .iter()
            .filter_map(|game| {
                game.pulls
                    .iter()
                    .all(|bag| reference.can_contain(bag))
                    .then_some(game.number)
            })
            .sum()
    }

    fn part2(input: &str) -> Self::Output {
        let games: Vec<Game> = input.lines().map(parse_game).collect();
        games.iter().map(|game| game.minimum_bag().power()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day2;
    use crate::Puzzle;

    const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn example1() {
        assert_eq!(Day2::part1(INPUT), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(Day2::part2(INPUT), 2286)
    }
}
