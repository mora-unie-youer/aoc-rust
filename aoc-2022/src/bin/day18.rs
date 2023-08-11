use std::collections::HashSet;

use aoc_2022::*;
use itertools::Itertools;

const DAY: i32 = 18;
type Solution = usize;

fn get_neighbors(x: isize, y: isize, z: isize) -> Vec<(isize, isize, isize)> {
    vec![
        (x - 1, y, z),
        (x, y - 1, z),
        (x, y, z - 1),
        (x + 1, y, z),
        (x, y + 1, z),
        (x, y, z + 1),
    ]
}

fn main() {
    let input = get_input_text(DAY);

    let cubes: HashSet<(isize, isize, isize)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',').map(|v| v.parse().unwrap());
            let x = parts.next().unwrap();
            let y = parts.next().unwrap();
            let z = parts.next().unwrap();
            (x, y, z)
        })
        .collect();

    let (min_x, max_x) = cubes
        .iter()
        .map(|&(x, _, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = cubes
        .iter()
        .map(|&(_, y, _)| y)
        .minmax()
        .into_option()
        .unwrap();
    let (min_z, max_z) = cubes
        .iter()
        .map(|&(_, _, z)| z)
        .minmax()
        .into_option()
        .unwrap();

    let solution1: Solution = {
        let mut surface_area = 0;
        for &(x, y, z) in &cubes {
            let neighbors = get_neighbors(x, y, z);
            surface_area += neighbors
                .into_iter()
                .filter(|pos| !cubes.contains(pos))
                .count();
        }

        surface_area
    };

    let solution2: Solution = {
        // Creating box around our lava
        let mut cubes = cubes;
        let (min_x, max_x) = (min_x - 2, max_x + 2);
        let (min_y, max_y) = (min_y - 2, max_y + 2);
        let (min_z, max_z) = (min_z - 2, max_z + 2);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                cubes.insert((x, y, min_z));
                cubes.insert((x, y, max_z));
            }

            for z in min_z..=max_z {
                cubes.insert((x, min_y, z));
                cubes.insert((x, max_y, z));
            }
        }

        for z in min_z..=max_z {
            for y in min_y..=max_y {
                cubes.insert((min_x, y, z));
                cubes.insert((max_x, y, z));
            }
        }

        // Filling free space between lava and box
        let mut queue = vec![(min_x + 1, min_y + 1, min_z + 1)];
        while let Some(pos @ (x, y, z)) = queue.pop() {
            if cubes.insert(pos) {
                let neighbors = get_neighbors(x, y, z);
                for neighbor in neighbors {
                    if !cubes.contains(&neighbor) {
                        queue.push(neighbor);
                    }
                }
            }
        }

        // Calculating surface area of this box
        let mut surface_area = 0;
        for &(x, y, z) in &cubes {
            let neighbors = get_neighbors(x, y, z);
            surface_area += neighbors
                .into_iter()
                .filter(|pos| !cubes.contains(pos))
                .count();
        }

        // Calculating inner area and final result
        let (x, y, z) = (max_x - min_x + 1, max_y - min_y + 1, max_z - min_z + 1);
        let expected_area = 2 * x * y + 2 * y * z + 2 * x * z;
        let inner_area = surface_area - expected_area as usize;

        solution1 - inner_area
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
