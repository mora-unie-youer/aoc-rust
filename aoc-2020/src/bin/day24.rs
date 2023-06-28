use std::{
    collections::HashSet,
    ops::{Add, AddAssign},
};

use aoc_2020::*;

const DAY: i32 = 24;
type Solution = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

fn parse_directions(line: &str) -> Vec<Direction> {
    let mut directions = Vec::new();
    let mut chars = line.chars();

    while let Some(ch) = chars.next() {
        let nch = if ch == 'n' || ch == 's' {
            chars.next()
        } else {
            None
        };

        let direction = match ch {
            'e' => Direction::East,
            'w' => Direction::West,
            'n' if nch == Some('e') => Direction::Northeast,
            'n' if nch == Some('w') => Direction::Northwest,
            's' if nch == Some('e') => Direction::Southeast,
            's' if nch == Some('w') => Direction::Southwest,
            _ => unreachable!(),
        };

        directions.push(direction);
    }

    directions
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize, isize);

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl From<(isize, isize, isize)> for Position {
    fn from(value: (isize, isize, isize)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

fn get_tile_coordinates(direction: Direction) -> Position {
    match direction {
        Direction::East => (1, -1, 0).into(),
        Direction::Southeast => (0, -1, 1).into(),
        Direction::Southwest => (-1, 0, 1).into(),
        Direction::West => (-1, 1, 0).into(),
        Direction::Northwest => (0, 1, -1).into(),
        Direction::Northeast => (1, 0, -1).into(),
    }
}

fn get_adjacent_coordinates(pos: &Position) -> Vec<Position> {
    let directions = [
        Direction::East,
        Direction::Southeast,
        Direction::Southwest,
        Direction::West,
        Direction::Northwest,
        Direction::Northeast,
    ];

    directions
        .iter()
        .map(|&direction| {
            let dir = get_tile_coordinates(direction);
            *pos + dir
        })
        .collect()
}

fn get_adjacent_black_count(tile: &Position, black_tiles: &HashSet<Position>) -> usize {
    get_adjacent_coordinates(tile)
        .iter()
        .filter(|&coord| black_tiles.contains(coord))
        .count()
}

fn main() {
    let input = get_input_text(DAY);

    let mut flipped_tiles = HashSet::new();
    for line in input.lines() {
        let directions = parse_directions(line);
        let mut pos = Position::default();

        for direction in directions {
            let dir = get_tile_coordinates(direction);
            pos += dir;
        }

        if flipped_tiles.contains(&pos) {
            flipped_tiles.remove(&pos);
        } else {
            flipped_tiles.insert(pos);
        }
    }

    let solution1: Solution = flipped_tiles.len();
    let solution2: Solution = {
        const DAYS: usize = 100;

        let mut black_tiles = flipped_tiles;
        for _ in 0..DAYS {
            let mut new_black_tiles = HashSet::new();
            let mut white_tiles_to_check = HashSet::new();

            for &tile in &black_tiles {
                let black_adjacent_count = get_adjacent_black_count(&tile, &black_tiles);
                if black_adjacent_count == 1 || black_adjacent_count == 2 {
                    new_black_tiles.insert(tile);
                }

                let adjacent_coordinates = get_adjacent_coordinates(&tile);
                for coord in adjacent_coordinates {
                    if !black_tiles.contains(&coord) {
                        white_tiles_to_check.insert(coord);
                    }
                }
            }

            for tile in white_tiles_to_check {
                let black_adjacent_count = get_adjacent_black_count(&tile, &black_tiles);
                if black_adjacent_count == 2 {
                    new_black_tiles.insert(tile);
                }
            }

            black_tiles = new_black_tiles;
        }

        black_tiles.len()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
