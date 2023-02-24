use std::{collections::HashSet, hash::Hash};

use aoc_2016::*;

const DAY: i32 = 1;
type Solution = isize;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::East => Self::North,
        }
    }

    fn right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::East => Self::South,
        }
    }
}

#[derive(Clone, Copy)]
struct Position {
    x: Solution,
    y: Solution,
    direction: Direction,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            direction: Direction::North,
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Position {}

impl Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Position {
    fn distance(&self) -> Solution {
        self.x.abs() + self.y.abs()
    }

    fn rotate(&mut self, instruction: &str) {
        self.direction = match instruction.chars().next().unwrap() {
            'L' => self.direction.left(),
            'R' => self.direction.right(),
            _ => unreachable!(),
        };
    }

    fn step(&mut self, step_size: isize) {
        match self.direction {
            Direction::North => self.y += step_size,
            Direction::South => self.y -= step_size,
            Direction::West => self.x -= step_size,
            Direction::East => self.x += step_size,
        }
    }
}

fn main() {
    let input = get_input_text(DAY);
    let instructions = input.trim().split(", ");

    let solution1: Solution = {
        let mut position = Position::default();
        instructions.clone().for_each(|instruction| {
            let step: Solution = instruction[1..].parse().unwrap();
            position.rotate(instruction);
            position.step(step);
        });
        position.distance()
    };

    let solution2: Solution = {
        let mut position = Position::default();
        let mut trail: HashSet<Position> = HashSet::new();
        trail.insert(position);

        'main: for instruction in instructions {
            let step: Solution = instruction[1..].parse().unwrap();
            position.rotate(instruction);
            for _ in 0..step {
                position.step(1);

                if trail.contains(&position) {
                    break 'main;
                }

                trail.insert(position);
            }
        }

        position.distance()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
