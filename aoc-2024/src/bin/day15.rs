use aoc_2024::*;
use itertools::Itertools;

const DAY: i32 = 15;
type Solution = usize;

fn push(
    grid: &mut [Vec<char>],
    ch: char,
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    dry_run: bool,
) -> bool {
    let (nx, ny) = (x + dx, y + dy);
    let tile = grid[ny as usize][nx as usize];

    match tile {
        '#' => false,

        '.' => {
            if !dry_run {
                grid[ny as usize][nx as usize] = ch;
                grid[y as usize][x as usize] = '.';
            }

            true
        }

        'O' => {
            // Try to push box in front
            if push(grid, tile, nx, ny, dx, dy, dry_run) {
                grid[ny as usize][nx as usize] = ch;
                grid[y as usize][x as usize] = '.';
                true
            } else {
                false
            }
        }

        '[' | ']' => match (dx, dy) {
            (_, 0) => {
                // Try to push box in front
                if push(grid, tile, nx, ny, dx, dy, dry_run) {
                    grid[ny as usize][nx as usize] = ch;
                    grid[y as usize][x as usize] = '.';
                    true
                } else {
                    false
                }
            }

            (0, _) => {
                // Second part of box
                let (part, npx) = if tile == '[' {
                    (']', nx + 1)
                } else {
                    ('[', nx - 1)
                };

                // Check if you can push the box (dry run)
                if !push(grid, tile, nx, ny, dx, dy, true) {
                    return false;
                }

                if !push(grid, part, npx, ny, dx, dy, true) {
                    return false;
                } else if dry_run {
                    return true;
                }

                // We can push boxes and we are not in dry run => push boxes
                push(grid, tile, nx, ny, dx, dy, false);
                push(grid, part, npx, ny, dx, dy, false);

                grid[ny as usize][nx as usize] = ch;
                grid[ny as usize][npx as usize] = '.';
                grid[y as usize][x as usize] = '.';
                true
            }

            _ => unreachable!(),
        },

        '@' => {
            dbg!("WTF");
            unreachable!()
        }
        _ => unreachable!(),
    }
}

fn main() {
    let input = get_input_text(DAY);
    let (grid, steps) = input.split_once("\n\n").unwrap();

    let grid = grid
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let steps = steps.chars().filter(|&ch| ch != '\n').collect_vec();

    let solution1: Solution = {
        let mut grid = grid.clone();
        let start = grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, ch)| (i, j, ch))
                    .find(|&(_, _, &ch)| ch == '@')
                    .map(|(i, j, _)| (i as isize, j as isize))
            })
            .next()
            .unwrap();

        let (mut ry, mut rx) = start;
        for &step in &steps {
            let (dx, dy) = match step {
                '<' => (-1, 0),
                '>' => (1, 0),
                '^' => (0, -1),
                'v' => (0, 1),
                _ => unreachable!(),
            };

            if push(&mut grid, '@', rx, ry, dx, dy, false) {
                rx += dx;
                ry += dy;
            }
        }

        grid.iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &ch)| ch == 'O')
                    .map(move |(j, _)| (i, j))
            })
            .map(|(i, j)| i * 100 + j)
            .sum()
    };

    let solution2: Solution = {
        let mut grid = grid
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .flat_map(|ch| match ch {
                        '#' => ['#', '#'],
                        '.' => ['.', '.'],
                        'O' => ['[', ']'],
                        '@' => ['@', '.'],
                        _ => unreachable!(),
                    })
                    .collect_vec()
            })
            .collect_vec();
        let start = grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, ch)| (i, j, ch))
                    .find(|&(_, _, &ch)| ch == '@')
                    .map(|(i, j, _)| (i as isize, j as isize))
            })
            .next()
            .unwrap();

        let (mut ry, mut rx) = start;
        for &step in &steps {
            let (dx, dy) = match step {
                '<' => (-1, 0),
                '>' => (1, 0),
                '^' => (0, -1),
                'v' => (0, 1),
                _ => unreachable!(),
            };

            if push(&mut grid, '@', rx, ry, dx, dy, false) {
                rx += dx;
                ry += dy;
            }
        }

        grid.iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &ch)| ch == '[')
                    .map(move |(j, _)| (i, j))
            })
            .map(|(i, j)| i * 100 + j)
            .sum()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
