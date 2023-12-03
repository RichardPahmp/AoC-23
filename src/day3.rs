use std::ops::Range;

use crate::Puzzle;

fn extend_bounds(bounds: Range<usize>) -> Range<usize> {
    (bounds.start.saturating_sub(1))..(bounds.end + 1)
}

#[derive(Debug)]
enum Part {
    Symbol { _symbol: char, index: usize },
    Number { number: u32, bounds: Range<usize> },
}

struct SchematicIterator<'a> {
    line: &'a [u8],
    idx: usize,
}

impl<'a> Iterator for SchematicIterator<'a> {
    type Item = Part;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.idx..self.line.len() {
            let part = if self.line[i] == b'.' {
                continue;
            } else if self.line[i].is_ascii_digit() {
                let mut offset = i + 1;
                while offset < self.line.len() && self.line[offset].is_ascii_digit() {
                    offset += 1;
                }
                let number = std::str::from_utf8(&self.line[i..offset])
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                let bounds = i..offset;
                self.idx = offset;
                Part::Number { number, bounds }
            } else {
                let symbol = self.line[i];
                self.idx = i + 1;
                Part::Symbol {
                    _symbol: symbol.into(),
                    index: i,
                }
            };

            return Some(part);
        }
        None
    }
}

#[derive(Debug)]
struct Schematic {
    rows: Vec<Vec<Part>>,
}

impl Schematic {
    pub fn parse(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| {
                let iter = SchematicIterator {
                    idx: 0,
                    line: line.as_bytes(),
                };
                iter.collect()
            })
            .collect();

        Self { rows }
    }

    pub fn iter_adjacent_rows(&self, row: usize) -> impl Iterator<Item = &Part> {
        (row.saturating_sub(1)..=(row + 1).min(self.rows.len() - 1)).flat_map(|i| &self.rows[i])
    }

    pub fn has_adjacent_symbol(&self, row: usize, range: Range<usize>) -> bool {
        let bounds = extend_bounds(range);
        let symbol_indices = self.iter_adjacent_rows(row).filter_map(|part| {
            if let Part::Symbol { index, .. } = part {
                Some(*index)
            } else {
                None
            }
        });

        for symbol in symbol_indices {
            if bounds.contains(&symbol) {
                return true;
            }
        }
        false
    }

    pub fn get_ratio(&self, row: usize, index: usize) -> Option<u32> {
        let adjacent_numbers: Vec<u32> = self
            .iter_adjacent_rows(row)
            .filter_map(|part| {
                if let Part::Number { number, bounds } = part {
                    Some((*number, bounds))
                } else {
                    None
                }
            })
            .filter_map(|(number, bounds)| {
                extend_bounds(bounds.clone())
                    .contains(&index)
                    .then_some(number)
            })
            .collect();

        if adjacent_numbers.len() == 2 {
            Some(adjacent_numbers.iter().product())
        } else {
            None
        }
    }
}

pub struct Day3;

impl Puzzle for Day3 {
    type Output = u32;

    fn part1(input: &str) -> Self::Output {
        let schematic = Schematic::parse(input);
        let mut total = 0;
        for (i, row) in schematic.rows.iter().enumerate() {
            for part in row {
                if let Part::Number { number, bounds } = part {
                    if schematic.has_adjacent_symbol(i, bounds.clone()) {
                        total += *number;
                    }
                }
            }
        }

        total
    }

    fn part2(input: &str) -> Self::Output {
        let schematic = Schematic::parse(input);
        let mut total = 0;
        for (i, row) in schematic.rows.iter().enumerate() {
            for part in row {
                if let Part::Symbol { index, .. } = part {
                    if let Some(ratio) = schematic.get_ratio(i, *index) {
                        total += ratio;
                    }
                }
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::Day3;
    use crate::Puzzle;

    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn example1() {
        assert_eq!(Day3::part1(INPUT), 4361);
    }

    #[test]
    fn example2() {
        assert_eq!(Day3::part2(INPUT), 467835);
    }
}
