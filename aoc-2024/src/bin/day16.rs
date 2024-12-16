use std::collections::HashSet;

use aoc_2024::*;
use itertools::Itertools;
use pathfinding::directed::yen;

const DAY: i32 = 16;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

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

    let paths = yen::yen(
        &(start.0, start.1, 0, 1),
        |&(y, x, dy, dx)| {
            let mut next = vec![];

            let (sdx, sdy) = (dx, dy);
            let (cdx, cdy) = (dy, -dx);
            let (ccdx, ccdy) = (-dy, dx);

            let (sx, sy) = (x + sdx, y + sdy);
            if grid[sy as usize][sx as usize] != '#' {
                next.push(((sy, sx, sdy, sdx), 1));
            }

            next.push(((y, x, cdy, cdx), 1000));
            next.push(((y, x, ccdy, ccdx), 1000));
            next
        },
        |&(y, x, _, _)| grid[y as usize][x as usize] == 'E',
        10,
    );

    let mut lowest = usize::MAX;
    let mut cells = HashSet::new();
    for (path, cost) in paths {
        if lowest < cost {
            break;
        }
        lowest = cost;
        cells.extend(path.into_iter().map(|(y, x, _, _)| (y, x)));
    }

    let solution1: Solution = lowest;
    let solution2: Solution = cells.len();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
