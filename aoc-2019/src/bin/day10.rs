use std::{
    collections::HashSet,
    ops::{Div, Sub},
};

use aoc_2019::*;

const DAY: i32 = 10;
type Solution = isize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize); // (x, y)

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Div<isize> for Position {
    type Output = Self;

    fn div(self, rhs: isize) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl Position {
    fn angle(&self) -> f64 {
        let angle = (self.1 as f64).atan2(self.0 as f64) + std::f64::consts::PI / 2.0;

        if angle < 0.0 {
            angle + 2.0 * std::f64::consts::PI
        } else {
            angle
        }
    }

    fn angle_to(&self, other: Self) -> f64 {
        let diff = *self - other;
        diff.angle()
    }

    fn distance(&self) -> isize {
        self.0 * self.0 + self.1 * self.1
    }

    fn distance_to(&self, other: Self) -> isize {
        let diff = *self - other;
        diff.distance()
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn count_visible_from(map: &HashSet<Position>, pos: Position) -> usize {
    let mut visible = HashSet::new();
    for &asteroid in map.iter() {
        if pos == asteroid {
            continue;
        }

        let diff = pos - asteroid;
        let factor = gcd(diff.0.unsigned_abs(), diff.1.unsigned_abs());
        let normalized = diff / factor as isize;
        visible.insert(normalized);
    }

    visible.len()
}

fn vaporize(map: &HashSet<Position>, base: Position, total_count: usize) -> Position {
    let mut asteroids: Vec<_> = map
        .iter()
        .filter(|&&pos| pos != base)
        .map(|asteroid| {
            let angle = asteroid.angle_to(base);
            let quantized_angle = (angle * 1_000.0) as isize;
            let distance = asteroid.distance_to(base);
            (quantized_angle, distance, asteroid)
        })
        .collect();
    asteroids.sort_by_key(|&(angle, distance, _)| (angle, distance));

    let mut i = 0;
    let mut count = 0;
    let mut last = (-1, 0, &Position(0, 0));

    while count < total_count {
        if i >= asteroids.len() {
            i = 0;
            last.0 = -1;
        } else if last.0 == asteroids[i].0 {
            i += 1;
            continue;
        }

        last = asteroids.remove(i);
        count += 1;
    }

    *last.2
}

fn main() {
    let input = get_input_text(DAY);

    let map: HashSet<Position> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| ch == '#')
                .map(move |(x, _)| Position(x as isize, y as isize))
        })
        .collect();

    let (best, count) = map
        .iter()
        .map(|&pos| (pos, count_visible_from(&map, pos)))
        .max_by_key(|&(_, count)| count)
        .unwrap();

    let solution1: Solution = count as isize;

    let solution2: Solution = {
        let last = vaporize(&map, best, 200);
        last.0 * 100 + last.1
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
