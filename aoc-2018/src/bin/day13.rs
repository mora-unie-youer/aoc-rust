use std::collections::{HashMap, HashSet};

use aoc_2018::*;

const DAY: i32 = 13;
type Solution = String;

#[derive(Clone)]
enum Cell {
    Track,
    Intersection,
    Curve(bool), // backslash: bool
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self, cell: Cell, turn: &mut Turn) -> Direction {
        match (self, cell) {
            (dir, Cell::Track) => *dir,
            (dir, Cell::Intersection) => turn.next(dir),
            (Direction::Up, Cell::Curve(false)) => Direction::Right,
            (Direction::Up, Cell::Curve(true)) => Direction::Left,
            (Direction::Down, Cell::Curve(false)) => Direction::Left,
            (Direction::Down, Cell::Curve(true)) => Direction::Right,
            (Direction::Left, Cell::Curve(false)) => Direction::Down,
            (Direction::Left, Cell::Curve(true)) => Direction::Up,
            (Direction::Right, Cell::Curve(false)) => Direction::Up,
            (Direction::Right, Cell::Curve(true)) => Direction::Down,
        }
    }
}

#[derive(Clone, Copy)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(&mut self, direction: &Direction) -> Direction {
        let next_direction = match (*self, direction) {
            (Turn::Left, Direction::Up) => Direction::Left,
            (Turn::Left, Direction::Left) => Direction::Down,
            (Turn::Left, Direction::Down) => Direction::Right,
            (Turn::Left, Direction::Right) => Direction::Up,
            (Turn::Straight, cart) => *cart,
            (Turn::Right, Direction::Up) => Direction::Right,
            (Turn::Right, Direction::Right) => Direction::Down,
            (Turn::Right, Direction::Down) => Direction::Left,
            (Turn::Right, Direction::Left) => Direction::Up,
        };

        *self = match *self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        };

        next_direction
    }
}

type Position = (usize, usize);
type Cart = (Position, Direction, Turn);

fn solve_part1(grid: &HashMap<Position, Cell>, mut carts: Vec<Cart>) -> Position {
    loop {
        carts.sort_by_key(|&(pos, _, _)| pos);

        let mut positions = HashSet::new();
        for &(pos, _, _) in &carts {
            if !positions.insert(pos) {
                return pos;
            }
        }

        for (pos, direction, turn) in &mut carts {
            positions.remove(pos);
            match *direction {
                Direction::Up => pos.1 -= 1,
                Direction::Down => pos.1 += 1,
                Direction::Left => pos.0 -= 1,
                Direction::Right => pos.0 += 1,
            }

            if !positions.insert(*pos) {
                return *pos;
            }

            *direction = direction.next(grid.get(pos).cloned().unwrap(), turn);
        }
    }
}

fn solve_part2(grid: &HashMap<Position, Cell>, mut carts: Vec<Cart>) -> Position {
    loop {
        if carts.len() == 1 {
            return carts[0].0;
        }

        carts.sort_by_key(|&(pos, _, _)| pos);

        let mut positions = HashSet::new();
        let mut remove = HashSet::new();
        for &(pos, _, _) in &carts {
            if !positions.insert(pos) {
                remove.insert(pos);
            }
        }

        for (pos, direction, turn) in &mut carts {
            if remove.contains(pos) {
                continue;
            }

            positions.remove(pos);
            match *direction {
                Direction::Up => pos.1 -= 1,
                Direction::Down => pos.1 += 1,
                Direction::Left => pos.0 -= 1,
                Direction::Right => pos.0 += 1,
            }

            if !positions.insert(*pos) {
                remove.insert(*pos);
                continue;
            }

            *direction = direction.next(grid.get(pos).cloned().unwrap(), turn);
        }

        if !remove.is_empty() {
            carts.retain(|(pos, _, _)| !remove.contains(pos));
        }
    }
}

fn main() {
    let input = get_input_text(DAY);

    let mut carts: Vec<Cart> = Vec::new();
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pos = (x, y);
            match ch {
                '+' => grid.insert(pos, Cell::Intersection),
                '/' => grid.insert(pos, Cell::Curve(false)),
                '\\' => grid.insert(pos, Cell::Curve(true)),
                '-' | '|' => grid.insert(pos, Cell::Track),
                '^' => {
                    carts.push((pos, Direction::Up, Turn::Left));
                    grid.insert(pos, Cell::Track)
                }
                'v' => {
                    carts.push((pos, Direction::Down, Turn::Left));
                    grid.insert(pos, Cell::Track)
                }
                '<' => {
                    carts.push((pos, Direction::Left, Turn::Left));
                    grid.insert(pos, Cell::Track)
                }
                '>' => {
                    carts.push((pos, Direction::Right, Turn::Left));
                    grid.insert(pos, Cell::Track)
                }
                ' ' => None,
                _ => unreachable!(),
            };
        }
    }

    dbg!(input);

    let solution1: Solution = {
        let (x, y) = solve_part1(&grid, carts.clone());
        format!("{},{}", x, y)
    };

    let solution2: Solution = {
        let (x, y) = solve_part2(&grid, carts.clone());
        format!("{},{}", x, y)
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
