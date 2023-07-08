use std::collections::{BinaryHeap, HashSet};

use aoc_2021::*;
use pathfinding::prelude::Matrix;

const DAY: i32 = 9;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let map = Matrix::from_rows(map).unwrap();

    let low_points: Vec<(usize, usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &cell)| (i, j, cell)))
        .filter(|&(i, j, cell)| map.neighbours((i, j), false).all(|v| map[v] > cell))
        .collect();

    let solution1: Solution = low_points.iter().map(|(_, _, cell)| 1 + cell).sum();
    let solution2: Solution = {
        let mut basins: Vec<usize> = low_points
            .into_iter()
            .map(|point| {
                let mut visited = HashSet::new();
                let mut queue = BinaryHeap::new();
                queue.push(point);

                while let Some((y, x, cell)) = queue.pop() {
                    if !visited.insert((y, x)) {
                        continue;
                    }

                    map.neighbours((y, x), false)
                        .filter(|&pos| map[pos] != 9 && map[pos] > cell)
                        .for_each(|(y, x)| queue.push((y, x, map[(y, x)])));
                }

                visited.len()
            })
            .collect();
        basins.sort();
        basins.iter().rev().take(3).product()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
