use std::collections::HashMap;

use aoc_2017::*;

const DAY: i32 = 22;
type Solution = usize;

#[derive(Clone, PartialEq)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl From<bool> for State {
    fn from(value: bool) -> Self {
        if value {
            Self::Infected
        } else {
            Self::Clean
        }
    }
}

impl State {
    fn infectful(&self, part2: bool) -> bool {
        let part1 = !part2 && matches!(self, Self::Clean);
        let part2 = part2 && matches!(self, Self::Weakened);
        part1 || part2
    }

    fn next(&self, part2: bool) -> Self {
        if part2 {
            match self {
                Self::Clean => Self::Weakened,
                Self::Weakened => Self::Infected,
                Self::Infected => Self::Flagged,
                Self::Flagged => Self::Clean,
            }
        } else {
            match self {
                Self::Clean => Self::Infected,
                Self::Infected => Self::Clean,
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Clone)]
struct Grid(HashMap<(isize, isize), State>);

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len();
        let height = value.lines().count();

        let (cx, cy) = (width as isize / 2, height as isize / 2);
        let mut grid = HashMap::new();
        value.lines().enumerate().for_each(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(i, pixel)| (i, pixel == '#'))
                .for_each(|(j, pixel)| {
                    grid.insert((j as isize - cx, i as isize - cy), State::from(pixel));
                })
        });

        Self(grid)
    }
}

fn solve(mut grid: Grid, bursts: usize, part2: bool) -> usize {
    let mut count = 0;
    let mut pos = (0, 0);
    let mut direction = (0, -1);

    for _ in 0..bursts {
        let state = grid.0.get(&pos).unwrap_or(&State::Clean);

        direction = match state {
            State::Clean => (direction.1, -direction.0),
            State::Weakened => direction,
            State::Infected => (-direction.1, direction.0),
            State::Flagged => (-direction.0, -direction.1),
        };

        if state.infectful(part2) {
            count += 1;
        }

        grid.0.insert(pos, state.next(part2));
        pos = (pos.0 + direction.0, pos.1 + direction.1);
    }

    count
}

fn main() {
    let input = get_input_text(DAY);
    let grid = Grid::from(&*input);

    let solution1: Solution = solve(grid.clone(), 10_000, false);
    let solution2: Solution = solve(grid, 10_000_000, true);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::{solve, Grid};

    #[test]
    fn test_solve() {
        let input = "..#\n#..\n...\n";
        let grid = Grid::from(input);
        assert_eq!(solve(grid.clone(), 10_000, false), 5587);
        assert_eq!(solve(grid, 10_000_000, true), 2511944);
    }
}
