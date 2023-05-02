use std::collections::HashMap;

use aoc_2018::*;

const DAY: i32 = 18;
type Solution = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Acre {
    Open,
    Tree,
    Lumberyard,
}

impl From<char> for Acre {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Open,
            '|' => Self::Tree,
            '#' => Self::Lumberyard,
            _ => unreachable!(),
        }
    }
}

impl Acre {
    fn is_tree(&self) -> bool {
        matches!(self, Self::Tree)
    }

    fn is_lumberyard(&self) -> bool {
        matches!(self, Self::Lumberyard)
    }

    fn next(&self, neighbors: &[Acre]) -> Acre {
        match self {
            Self::Open => {
                let trees = neighbors.iter().filter(|acre| acre.is_tree()).count();
                if trees >= 3 {
                    Self::Tree
                } else {
                    Self::Open
                }
            }
            Self::Tree => {
                let lumberyards = neighbors.iter().filter(|acre| acre.is_lumberyard()).count();
                if lumberyards >= 3 {
                    Self::Lumberyard
                } else {
                    Self::Tree
                }
            }
            Self::Lumberyard => {
                let lumberyard = neighbors.iter().find(|acre| acre.is_lumberyard());
                let tree = neighbors.iter().find(|acre| acre.is_tree());
                if lumberyard.is_some() && tree.is_some() {
                    Self::Lumberyard
                } else {
                    Self::Open
                }
            }
        }
    }
}

fn tick(grid: Vec<Vec<Acre>>) -> Vec<Vec<Acre>> {
    let mut new_grid = vec![];
    for y in 0..grid.len() {
        let mut row = vec![];
        for x in 0..grid[0].len() {
            let min_x = x.saturating_sub(1);
            let min_y = y.saturating_sub(1);
            let max_x = (x + 1).min(grid[0].len() - 1);
            let max_y = (y + 1).min(grid.len() - 1);

            let mut neighbors = vec![];
            for (ny, row) in grid.iter().enumerate().take(max_y + 1).skip(min_y) {
                for (nx, acre) in row.iter().enumerate().take(max_x + 1).skip(min_x) {
                    if ny != y || nx != x {
                        neighbors.push(*acre);
                    }
                }
            }

            row.push(grid[y][x].next(&neighbors));
        }

        new_grid.push(row);
    }

    new_grid
}

fn main() {
    let input = get_input_text(DAY);

    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(Acre::from).collect())
        .collect();

    let solution1: Solution = {
        let grid = (0..10).fold(grid.clone(), |acc, _| tick(acc));
        let lumberyards = grid
            .iter()
            .flatten()
            .filter(|acre| acre.is_lumberyard())
            .count();
        let trees = grid.iter().flatten().filter(|acre| acre.is_tree()).count();
        lumberyards * trees
    };

    const CYCLES: usize = 1_000_000_000;
    let solution2: Solution = {
        let mut grid = grid;
        let mut seen = HashMap::new();
        let mut iteration = 0;
        while iteration < CYCLES {
            if let Some(i) = seen.get(&grid) {
                let cycle_step = iteration - i;
                iteration += (CYCLES - iteration) / cycle_step * cycle_step;
            } else {
                seen.insert(grid.clone(), iteration);
            }

            grid = tick(grid);
            iteration += 1;
        }

        let lumberyards = grid
            .iter()
            .flatten()
            .filter(|acre| acre.is_lumberyard())
            .count();
        let trees = grid.iter().flatten().filter(|acre| acre.is_tree()).count();
        lumberyards * trees
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
