#![allow(clippy::needless_range_loop)]
#![feature(let_chains)]

use aoc_2018::*;

const DAY: i32 = 17;
type Solution = usize;

enum Coordinate {
    Single(usize),
    Range(usize, usize),
}

impl Coordinate {
    fn start(&self) -> usize {
        match *self {
            Self::Single(v) => v,
            Self::Range(start, _) => start,
        }
    }

    fn end(&self) -> usize {
        match *self {
            Self::Single(v) => v,
            Self::Range(_, end) => end,
        }
    }
}

impl From<&str> for Coordinate {
    fn from(value: &str) -> Self {
        if let Some((start, end)) = value.split_once("..") {
            Self::Range(start.parse().unwrap(), end.parse().unwrap())
        } else {
            Self::Single(value.parse().unwrap())
        }
    }
}

struct Vein(Coordinate, Coordinate);

impl From<&str> for Vein {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(", ").unwrap();
        let (x, y) = if x.starts_with('y') { (y, x) } else { (x, y) };
        Self(x[2..].into(), y[2..].into())
    }
}

fn drip(x_start: usize, y_start: usize, grid: &mut Vec<Vec<char>>) -> bool {
    for y_fall in y_start..grid.len() {
        if grid[y_fall][x_start] == '.' {
            grid[y_fall][x_start] = '|';
        }

        if grid[y_fall][x_start] == '|' {
            continue;
        }

        // Hit something
        let y_level = y_fall - 1;
        let (mut x_left, mut x_right) = (None, None);
        let mut dripped = false;

        for x in (0..x_start).rev() {
            if grid[y_level][x] == '#' {
                x_left = Some(x + 1);
                break;
            }

            grid[y_level][x] = '|';

            if grid[y_fall][x] == '.' || grid[y_fall][x] == '|' {
                dripped |= drip(x, y_level, grid);
                break;
            }
        }

        for x in x_start..grid[0].len() {
            if grid[y_level][x] == '#' {
                x_right = Some(x - 1);
                break;
            }

            grid[y_level][x] = '|';

            if grid[y_fall][x] == '.' || grid[y_fall][x] == '|' {
                dripped |= drip(x, y_level, grid);
                break;
            }
        }

        if let Some(start) = x_left && let Some(end) = x_right {
            dripped = true;
            for x in start..=end {
                grid[y_level][x] = '~';
            }
        }

        return dripped;
    }

    false
}

fn main() {
    let input = get_input_text(DAY);
    let veins: Vec<_> = input.lines().map(Vein::from).collect();

    let (_min_x, min_y, max_x, max_y) = {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (
            std::usize::MAX,
            std::usize::MAX,
            std::usize::MIN,
            std::usize::MIN,
        );

        for vein in &veins {
            min_x = min_x.min(vein.0.start());
            min_y = min_y.min(vein.1.start());
            max_x = max_x.max(vein.0.end());
            max_y = max_y.max(vein.1.end());
        }

        (min_x, min_y, max_x, max_y)
    };

    let mut grid = vec![vec!['.'; max_x + 1]; max_y + 1];
    for vein in &veins {
        for y in vein.1.start()..=vein.1.end() {
            for x in vein.0.start()..=vein.0.end() {
                grid[y][x] = '#';
            }
        }
    }

    loop {
        if !drip(500, 0, &mut grid) {
            break;
        }
    }

    let solution1: Solution = grid
        .iter()
        .skip(min_y)
        .flatten()
        .filter(|&&ch| ch == '~' || ch == '|')
        .count();
    let solution2: Solution = grid
        .iter()
        .skip(min_y)
        .flatten()
        .filter(|&&ch| ch == '~')
        .count();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
