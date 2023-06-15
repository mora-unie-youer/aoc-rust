use aoc_2020::*;
use pathfinding::prelude::Matrix;

const DAY: i32 = 11;
type Solution = usize;
type Cell = Option<bool>;

fn tick_part1(grid: &Matrix<Cell>) -> Matrix<Cell> {
    let mut new_grid = grid.clone();

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let neighbors = grid
                .neighbours((y, x), true)
                .filter(|&v| grid[v].unwrap_or(false))
                .count();

            new_grid[(y, x)] = match *cell {
                Some(false) if neighbors == 0 => Some(true),
                Some(true) if neighbors >= 4 => Some(false),
                v => v,
            };
        }
    }

    new_grid
}

fn count_neighbors(grid: &Matrix<Cell>, x: usize, y: usize) -> usize {
    let mut count = 0;
    let grid_bounds =
        |(y, x)| y >= 0 && y < grid.rows as isize && x >= 0 && x < grid.columns as isize;
    let grid_cell = |(y, x)| grid[(y as usize, x as usize)];

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let mut pos = (y as isize + dy, x as isize + dx);
            while grid_bounds(pos) && grid_cell(pos).is_none() {
                pos = (pos.0 + dy, pos.1 + dx);
            }

            if !grid_bounds(pos) {
                continue;
            }

            count += match grid_cell(pos) {
                Some(true) => 1,
                Some(false) => 0,
                None => unreachable!(),
            };
        }
    }

    count
}

fn tick_part2(grid: &Matrix<Cell>) -> Matrix<Cell> {
    let mut new_grid = grid.clone();

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let neighbors = count_neighbors(grid, x, y);

            new_grid[(y, x)] = match *cell {
                Some(false) if neighbors == 0 => Some(true),
                Some(true) if neighbors >= 5 => Some(false),
                v => v,
            };
        }
    }

    new_grid
}

fn solve(mut grid: Matrix<Cell>, part2: bool) -> Solution {
    loop {
        let new_grid = if part2 {
            tick_part2(&grid)
        } else {
            tick_part1(&grid)
        };

        if grid == new_grid {
            break;
        }

        grid = new_grid;
    }

    grid.iter().flatten().filter(|v| v.unwrap_or(false)).count()
}

fn main() {
    let input = get_input_text(DAY);
    let grid: Matrix<Cell> = Matrix::from_rows(input.lines().map(|line| {
        line.chars().map(|ch| match ch {
            '.' => None,
            'L' => Some(false),
            '#' => Some(true),
            _ => unreachable!(),
        })
    }))
    .unwrap();

    let solution1: Solution = solve(grid.clone(), false);
    let solution2: Solution = solve(grid, true);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
