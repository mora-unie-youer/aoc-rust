use std::collections::HashSet;

use aoc_2015::*;

const DAY: i32 = 3;
type Solution = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i32, i32);

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = {
        let mut santa = Position(0, 0);
        let mut visited = HashSet::new();
        visited.insert(santa);

        input.chars().for_each(|ch| {
            match ch {
                '^' => santa.1 += 1,
                'v' => santa.1 -= 1,
                '<' => santa.0 -= 1,
                '>' => santa.0 += 1,
                _ => unreachable!(),
            }

            visited.insert(santa);
        });

        visited.len()
    };

    let solution2: Solution = {
        let (mut santa, mut robo_santa) = (Position(0, 0), Position(0, 0));
        let mut visited = HashSet::new();
        visited.insert(santa);

        input.char_indices().for_each(|(i, ch)| {
            let robo = i % 2 == 1;
            match ch {
                '^' if !robo => santa.1 += 1,
                'v' if !robo => santa.1 -= 1,
                '<' if !robo => santa.0 -= 1,
                '>' if !robo => santa.0 += 1,
                '^' => robo_santa.1 += 1,
                'v' => robo_santa.1 -= 1,
                '<' => robo_santa.0 -= 1,
                '>' => robo_santa.0 += 1,
                _ => unreachable!(),
            }

            visited.insert(if robo { robo_santa } else { santa });
        });

        visited.len()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
