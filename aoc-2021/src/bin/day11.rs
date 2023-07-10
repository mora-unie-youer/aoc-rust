use std::collections::VecDeque;

use aoc_2021::*;
use pathfinding::prelude::Matrix;

const DAY: i32 = 11;
type Solution = usize;

fn tick_map(map: &mut Matrix<usize>) -> usize {
    map.iter_mut().for_each(|v| *v += 1);

    let mut flashed = 0;
    let mut queue: VecDeque<(usize, usize)> = map
        .items()
        .filter(|&(_, &value)| value == 10)
        .map(|(pos, _)| pos)
        .collect();
    while let Some(pos) = queue.pop_front() {
        flashed += 1;
        for neighbor in map.neighbours(pos, true) {
            let cell = &mut map[neighbor];
            *cell += 1;

            if *cell == 10 {
                queue.push_back(neighbor);
            }
        }
    }

    map.iter_mut().for_each(|v| {
        if *v > 9 {
            *v = 0;
        }
    });

    flashed
}

fn main() {
    let input = get_input_text(DAY);

    let map = Matrix::from_rows(
        input
            .lines()
            .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as usize)),
    )
    .unwrap();

    let solution1: Solution = {
        const STEPS: usize = 100;

        let mut map = map.clone();
        let mut flashed = 0;
        for _ in 0..STEPS {
            flashed += tick_map(&mut map);
        }

        flashed
    };

    let solution2: Solution = {
        let mut map = map;
        let mut steps = 1;
        while tick_map(&mut map) != map.len() {
            steps += 1;
        }

        steps
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
