use std::collections::HashSet;

use aoc_2020::*;
use itertools::Itertools;

const DAY: i32 = 17;
type Solution = usize;

const CYCLES: usize = 6;
fn main() {
    let input = get_input_text(DAY);
    let slice: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch == '#').collect())
        .collect();

    let solution1: Solution = {
        let neighbors: Vec<(isize, isize, isize)> = (0..27)
            .filter(|&pos| pos != 27 / 2)
            .map(|pos| (pos % 3 - 1, pos / 3 % 3 - 1, pos / 9 - 1))
            .collect();
        let mut current_space: HashSet<(isize, isize, isize)> = HashSet::new();
        let mut previous_space;

        // Read map
        for (y, row) in slice.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell {
                    let offset = slice.len() as isize / 2;
                    current_space.insert((x as isize - offset, y as isize - offset, 0));
                }
            }
        }

        for _ in 0..CYCLES {
            previous_space = current_space;
            current_space = HashSet::new();

            let (x_min, x_max) = previous_space
                .iter()
                .map(|pos| pos.0)
                .minmax()
                .into_option()
                .unwrap();
            let (y_min, y_max) = previous_space
                .iter()
                .map(|pos| pos.1)
                .minmax()
                .into_option()
                .unwrap();
            let (z_min, z_max) = previous_space
                .iter()
                .map(|pos| pos.2)
                .minmax()
                .into_option()
                .unwrap();

            for z in z_min - 1..=z_max + 1 {
                for y in y_min - 1..=y_max + 1 {
                    for x in x_min - 1..=x_max + 1 {
                        let count = neighbors
                            .iter()
                            .map(|(dx, dy, dz)| (x + dx, y + dy, z + dz))
                            .filter(|pos| previous_space.contains(pos))
                            .count();

                        match previous_space.contains(&(x, y, z)) {
                            true if count == 2 || count == 3 => current_space.insert((x, y, z)),
                            false if count == 3 => current_space.insert((x, y, z)),
                            _ => false,
                        };
                    }
                }
            }
        }

        current_space.len()
    };

    let solution2: Solution = {
        let neighbors: Vec<(isize, isize, isize, isize)> = (0..81)
            .filter(|&pos| pos != 81 / 2)
            .map(|pos| (pos % 3 - 1, pos / 3 % 3 - 1, pos / 9 % 3 - 1, pos / 27 - 1))
            .collect();
        let mut current_space: HashSet<(isize, isize, isize, isize)> = HashSet::new();
        let mut previous_space;

        // Read map
        for (y, row) in slice.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell {
                    let offset = slice.len() as isize / 2;
                    current_space.insert((x as isize - offset, y as isize - offset, 0, 0));
                }
            }
        }

        for _ in 0..CYCLES {
            previous_space = current_space;
            current_space = HashSet::new();

            let (x_min, x_max) = previous_space
                .iter()
                .map(|pos| pos.0)
                .minmax()
                .into_option()
                .unwrap();
            let (y_min, y_max) = previous_space
                .iter()
                .map(|pos| pos.1)
                .minmax()
                .into_option()
                .unwrap();
            let (z_min, z_max) = previous_space
                .iter()
                .map(|pos| pos.2)
                .minmax()
                .into_option()
                .unwrap();
            let (w_min, w_max) = previous_space
                .iter()
                .map(|pos| pos.3)
                .minmax()
                .into_option()
                .unwrap();

            for w in w_min - 1..=w_max + 1 {
                for z in z_min - 1..=z_max + 1 {
                    for y in y_min - 1..=y_max + 1 {
                        for x in x_min - 1..=x_max + 1 {
                            let count = neighbors
                                .iter()
                                .map(|(dx, dy, dz, dw)| (x + dx, y + dy, z + dz, w + dw))
                                .filter(|pos| previous_space.contains(pos))
                                .count();

                            match previous_space.contains(&(x, y, z, w)) {
                                true if count == 2 || count == 3 => {
                                    current_space.insert((x, y, z, w))
                                }
                                false if count == 3 => current_space.insert((x, y, z, w)),
                                _ => false,
                            };
                        }
                    }
                }
            }
        }

        current_space.len()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
