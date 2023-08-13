use aoc_2022::*;
use itertools::Itertools;

const DAY: i32 = 20;
type Solution = isize;

fn main() {
    let input = get_input_text(DAY);
    let numbers: Vec<isize> = input.lines().map(|v| v.parse().unwrap()).collect();

    let solution1: Solution = {
        let mut indices = (0..numbers.len()).collect_vec();

        for (i, &num) in numbers.iter().enumerate() {
            let position = indices.iter().position(|&j| j == i).unwrap();
            indices.remove(position);

            let new_position =
                (position as isize + num).rem_euclid(indices.len() as isize) as usize;
            indices.insert(new_position, i);
        }

        let zero = indices.iter().position(|&i| numbers[i] == 0).unwrap();
        [1000, 2000, 3000]
            .iter()
            .map(|i| indices[(zero + i) % indices.len()])
            .map(|i| numbers[i])
            .sum()
    };

    let solution2: Solution = {
        const KEY: isize = 811589153;
        let numbers = numbers.into_iter().map(|v| v * KEY).collect_vec();
        let mut indices = (0..numbers.len()).collect_vec();

        for _ in 0..10 {
            for (i, &num) in numbers.iter().enumerate() {
                let position = indices.iter().position(|&j| j == i).unwrap();
                indices.remove(position);

                let new_position =
                    (position as isize + num).rem_euclid(indices.len() as isize) as usize;
                indices.insert(new_position, i);
            }
        }

        let zero = indices.iter().position(|&i| numbers[i] == 0).unwrap();
        [1000, 2000, 3000]
            .iter()
            .map(|i| indices[(zero + i) % indices.len()])
            .map(|i| numbers[i])
            .sum()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
