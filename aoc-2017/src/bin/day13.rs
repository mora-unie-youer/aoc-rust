use std::collections::HashMap;

use aoc_2017::*;

const DAY: i32 = 13;
type Solution = usize;

fn caught(depth: usize, range: usize) -> bool {
    depth % (2 * (range - 1)) == 0
}

fn main() {
    let input = get_input_text(DAY);
    let layers: HashMap<usize, usize> = input
        .lines()
        .map(|line| {
            let (depth, range) = line.split_once(": ").unwrap();
            (depth.parse().unwrap(), range.parse().unwrap())
        })
        .collect();

    let solution1: Solution = layers
        .iter()
        .filter(|(&depth, &range)| caught(depth, range))
        .map(|(depth, range)| depth * range)
        .sum();

    let solution2: Solution = {
        let mut delay = 0;
        loop {
            if layers
                .iter()
                .all(|(&depth, &range)| !caught(depth + delay, range))
            {
                break delay;
            }

            delay += 1;
        }
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
