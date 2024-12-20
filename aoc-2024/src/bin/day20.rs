use std::collections::BinaryHeap;

use aoc_2024::*;
use itertools::Itertools;

const DAY: i32 = 20;
type Solution = usize;

fn neighbors(grid: &[Vec<char>], y: isize, x: isize) -> Vec<(isize, isize)> {
    let width = grid[0].len() as isize;
    let height = grid.len() as isize;

    let cells = [(y, x - 1), (y, x + 1), (y - 1, x), (y + 1, x)];
    cells
        .into_iter()
        .filter(|&(y, x)| x >= 0 && x < width && y >= 0 && y < height)
        .filter(|&(y, x)| grid[y as usize][x as usize] != '#')
        // .filter(|&(y, x)| !cheated || grid[y as usize][x as usize] != '#')
        // .map(|(y, x)| (y, x, cheated || grid[y as usize][x as usize] == '#'))
        .collect_vec()
}

fn solve(grid: &[Vec<char>], cheat_time: isize) -> Solution {
    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, ch)| (i, j, ch))
                .find(|&(_, _, &ch)| ch == 'S')
                .map(|(i, j, _)| (i as isize, j as isize))
        })
        .next()
        .unwrap();

    let (width, height) = (grid[0].len(), grid.len());
    let mut costs = vec![vec![0; width]; height];

    let mut queue = BinaryHeap::from([(0, (start.0, start.1))]);
    while let Some((cost, (y, x))) = queue.pop() {
        if costs[y as usize][x as usize] != 0 {
            continue;
        } else {
            costs[y as usize][x as usize] = cost;
        }

        let next_neighbors = neighbors(grid, y, x).into_iter().map(|s| (cost + 1, s));
        queue.extend(next_neighbors);
    }

    let mut count = 0;
    for (y, row) in costs.iter().enumerate() {
        for (x, &cost) in row.iter().enumerate() {
            if cost == 0 {
                continue;
            }

            for r in -cheat_time..=cheat_time {
                for c in -(cheat_time - r.abs())..=(cheat_time - r.abs()) {
                    let dist = r.abs() + c.abs();
                    let (py, px) = (y as isize + r, x as isize + c);

                    if py < 0 || py >= (height as isize) || px < 0 || px >= (width as isize) {
                        continue;
                    }

                    if costs[py as usize][px as usize] >= cost + dist + 100 {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn main() {
    let input = get_input_text(DAY);

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let solution1: Solution = solve(&grid, 2);
    let solution2: Solution = solve(&grid, 20);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
