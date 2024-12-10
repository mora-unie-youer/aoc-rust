use std::collections::{BinaryHeap, HashSet};

use aoc_2024::*;
use itertools::Itertools;

const DAY: i32 = 10;
type Solution = usize;

fn get_neighbors(map: &[Vec<u8>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];

    if x > 0 {
        neighbors.push((x - 1, y));
    }

    if y > 0 {
        neighbors.push((x, y - 1));
    }

    if x < map[0].len() - 1 {
        neighbors.push((x + 1, y));
    }

    if y < map.len() - 1 {
        neighbors.push((x, y + 1));
    }

    neighbors
}

fn main() {
    let input = get_input_text(DAY);

    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.bytes().map(|ch| ch - b'0').collect())
        .collect();

    let start_points = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, cell)| *cell == 0)
                .map(move |(j, _)| (j, i))
        })
        .collect_vec();

    let (scores, ratings) = {
        let mut score = 0;
        let mut rating = 0;

        let mut queue: BinaryHeap<_> = start_points
            .iter()
            .map(|&(x, y)| (0, x, y, (x, y)))
            .collect();
        let mut visited = HashSet::new();
        while let Some((height, x, y, start_point)) = queue.pop() {
            if height == 9 {
                score += visited.insert((x, y, start_point)) as usize;
                rating += 1;
                continue;
            }

            let all_neighbors = get_neighbors(&map, x, y);
            let next_cells = all_neighbors
                .into_iter()
                .filter(|&(x, y)| map[y][x] == height + 1)
                .map(|(x, y)| (height + 1, x, y, start_point));
            queue.extend(next_cells);
        }

        (score, rating)
    };

    let solution1: Solution = scores;
    let solution2: Solution = ratings;

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
