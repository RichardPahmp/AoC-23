use crate::Puzzle;
use glam::UVec2;
use itertools::Itertools;
use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn step(self, pos: UVec2) -> Option<UVec2> {
        match self {
            Direction::Up => pos.y.checked_sub(1).map(|y| UVec2::new(pos.x, y)),
            Direction::Down => pos.y.checked_add(1).map(|y| UVec2::new(pos.x, y)),
            Direction::Left => pos.x.checked_sub(1).map(|x| UVec2::new(x, pos.y)),
            Direction::Right => pos.x.checked_add(1).map(|x| UVec2::new(x, pos.y)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Some(Direction, Direction),
    Start,
    None,
}

impl Tile {
    pub fn from_char(ch: char) -> Self {
        match ch {
            '|' => Self::Some(Direction::Up, Direction::Down),
            'J' => Self::Some(Direction::Up, Direction::Left),
            'L' => Self::Some(Direction::Up, Direction::Right),
            '7' => Self::Some(Direction::Left, Direction::Down),
            'F' => Self::Some(Direction::Right, Direction::Down),
            '-' => Self::Some(Direction::Left, Direction::Right),
            '.' => Self::None,
            'S' => Self::Start,
            _ => panic!("Invalid tile: {:?}", ch),
        }
    }

    pub fn adjacent(self, pos: UVec2) -> Option<(UVec2, UVec2)> {
        match self {
            Tile::Some(a, b) => {
                let a = a.step(pos)?;
                let b = b.step(pos)?;
                Some((a, b))
            }
            Tile::Start => None,
            Tile::None => None,
        }
    }

    pub fn connects_to(self, pos: UVec2, other: UVec2) -> bool {
        let Some((a, b)) = self.adjacent(pos) else {
            return false;
        };
        a == other || b == other
    }

    pub fn matches(self, (x, y): (Direction, Direction)) -> bool {
        if let Self::Some(a, b) = self {
            (a == x && b == y) || (b == x && a == y)
        } else {
            false
        }
    }
}

struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn from_iter(iter: impl Iterator<Item = T>, width: usize) -> Self {
        let data = iter.collect::<Vec<_>>();
        assert!(data.len() % width == 0);
        let height = data.len() / width;
        Self {
            data,
            width,
            height,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    fn find<P: Fn(&T) -> bool>(&self, predicate: P) -> Option<UVec2> {
        let idx = self.data.iter().position(predicate)?;
        let x = idx % self.width;
        let y = idx / self.width;
        Some(UVec2::new(x as u32, y as u32))
    }

    fn walk_diagonal(&self, from: UVec2) -> impl Iterator<Item = UVec2> {
        let dx = self.width as u32 - from.x;
        let dy = self.height as u32 - from.y;
        (0..dx.min(dy)).map(move |i| from + UVec2::new(i, i))
    }

    fn iter_positions(&self) -> impl Iterator<Item = UVec2> {
        (0..self.width)
            .cartesian_product(0..self.height)
            .map(|(x, y)| UVec2::new(x as u32, y as u32))
    }
}

impl<T> Index<(u32, u32)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        &self.data[self.index(x as usize, y as usize)]
    }
}

impl<T> IndexMut<(u32, u32)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
        let idx = self.index(x as usize, y as usize);
        &mut self.data[idx]
    }
}

pub struct Day10;

impl Puzzle for Day10 {
    type Output = usize;

    fn part1(input: &str) -> Self::Output {
        let width = input.lines().next().unwrap().len();
        let grid = Grid::from_iter(
            input.lines().flat_map(str::chars).map(Tile::from_char),
            width,
        );
        let start = grid.find(|t| matches!(t, Tile::Start)).unwrap();

        let neighbors = [
            (start.x - 1, start.y),
            (start.x + 1, start.y),
            (start.x, start.y + 1),
            (start.x, start.y - 1),
        ];
        let next = neighbors
            .iter()
            .find_map(|(x, y)| {
                let x = *x;
                let y = *y;
                let tile = grid[(x, y)];
                let pos = UVec2::new(x, y);
                let (left, right) = tile.adjacent(pos)?;
                if start == left || start == right {
                    Some(pos)
                } else {
                    None
                }
            })
            .unwrap();

        let mut pipes = HashSet::new();
        pipes.insert(start);

        let mut prev = start;
        let mut pos = next;
        while pos != start {
            pipes.insert(pos);
            let tile = grid[(pos.x, pos.y)];
            let (left, right) = tile.adjacent(pos).unwrap();
            let next = if left != prev { left } else { right };

            prev = pos;
            pos = next;
        }
        pipes.len() / 2
    }

    fn part2(input: &str) -> Self::Output {
        let width = input.lines().next().unwrap().len();
        let mut grid = Grid::from_iter(
            input.lines().flat_map(str::chars).map(Tile::from_char),
            width,
        );
        let start = grid.find(|t| matches!(t, Tile::Start)).unwrap();

        let mut neighbor_dirs = [
            Direction::Up,
            Direction::Left,
            Direction::Right,
            Direction::Down,
        ]
        .into_iter()
        .filter(|dir| {
            dir.step(start)
                .filter(|nb| grid[(nb.x, nb.y)].connects_to(*nb, start))
                .is_some()
        });
        let dir_a = neighbor_dirs.next().unwrap();
        let dir_b = neighbor_dirs.next().unwrap();
        assert!(neighbor_dirs.next().is_none());

        grid[start.into()] = Tile::Some(dir_a, dir_b);
        let start_a = dir_a.step(start).unwrap();
        let _start_b = dir_b.step(start).unwrap();

        let mut pipes = HashSet::new();
        pipes.insert(start);

        let mut prev = start;
        let mut pos = start_a;
        while pos != start {
            pipes.insert(pos);
            let tile = grid[(pos.x, pos.y)];
            let (left, right) = tile.adjacent(pos).unwrap();
            let next = if left != prev { left } else { right };

            prev = pos;
            pos = next;
        }

        grid.iter_positions()
            .filter(|pos| !pipes.contains(pos))
            .filter(|pos| {
                let mut count = 0;

                for p in grid.walk_diagonal(*pos) {
                    let current = grid[(p.x, p.y)];
                    if pipes.contains(&p)
                        && !current.matches((Direction::Up, Direction::Right))
                        && !current.matches((Direction::Left, Direction::Down))
                    {
                        count += 1;
                    }
                }

                count % 2 == 1
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"..F7.
.FJ|.
FS.L7
|F--J
LJ..."#;

    #[test]
    pub fn example1() {
        assert_eq!(Day10::part1(EXAMPLE), 8);
    }

    const EXAMPLE2: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;

    #[test]
    pub fn example2() {
        assert_eq!(Day10::part2(EXAMPLE2), 8);
    }

    const EXAMPLE3: &str = r#"......
.S--7.
.|..|.
.|..|.
.L--J.
......"#;

    #[test]
    fn example3() {
        assert_eq!(Day10::part2(EXAMPLE3), 4);
    }
}
