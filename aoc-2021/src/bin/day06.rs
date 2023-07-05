use std::collections::VecDeque;

use aoc_2021::*;

const DAY: i32 = 6;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let timers: Vec<usize> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let solution1: Solution = {
        const DAYS: usize = 80;
        let mut timers = timers.clone();

        for _ in 0..DAYS {
            for i in 0..timers.len() {
                if timers[i] == 0 {
                    timers[i] = 6;
                    timers.push(8);
                } else {
                    timers[i] -= 1;
                }
            }
        }

        timers.len()
    };

    let solution2: Solution = {
        const DAYS: usize = 256;

        // Cycle of timers
        let mut timers: VecDeque<usize> = (0..=6)
            .map(|v| timers.iter().filter(|&&time| time == v).count())
            .collect();
        let mut new_timers = VecDeque::from([0, 0]);

        for _ in 0..DAYS {
            let to_create = *timers.front().unwrap();
            let add_from_new = new_timers.pop_front().unwrap();
            *timers.front_mut().unwrap() += add_from_new;
            timers.rotate_left(1);
            new_timers.push_back(to_create);
        }

        let timers: usize = timers.into_iter().sum();
        let new_timers: usize = new_timers.into_iter().sum();
        timers + new_timers
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
