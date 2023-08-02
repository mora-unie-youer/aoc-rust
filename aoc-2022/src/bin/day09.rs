use std::collections::HashSet;

use aoc_2022::*;

const DAY: i32 = 9;
type Solution = usize;

type Position = (i32, i32);
fn follow(head: Position, tail: &mut Position) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    if dx.abs() > 1 || dy.abs() > 1 {
        tail.0 += dx.signum();
        tail.1 += dy.signum();
    }
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = {
        let mut visited = HashSet::new();
        let mut head = (0, 0);
        let mut tail = head;

        for instruction in input.lines() {
            let (direction, count) = instruction.split_once(' ').unwrap();
            let count: usize = count.parse().unwrap();
            for _ in 0..count {
                match direction {
                    "U" => head.1 += 1,
                    "D" => head.1 -= 1,
                    "L" => head.0 -= 1,
                    "R" => head.0 += 1,
                    _ => unreachable!(),
                }

                follow(head, &mut tail);
                visited.insert(tail);
            }
        }

        visited.len()
    };

    let solution2: Solution = {
        let mut visited = HashSet::new();
        let mut head = (0, 0);
        let mut tail = [Position::default(); 9];

        for instruction in input.lines() {
            let (direction, count) = instruction.split_once(' ').unwrap();
            let count: usize = count.parse().unwrap();
            for _ in 0..count {
                match direction {
                    "U" => head.1 += 1,
                    "D" => head.1 -= 1,
                    "L" => head.0 -= 1,
                    "R" => head.0 += 1,
                    _ => unreachable!(),
                }

                follow(head, &mut tail[0]);
                for i in 1..9 {
                    follow(tail[i - 1], &mut tail[i]);
                }

                visited.insert(tail[8]);
            }
        }

        visited.len()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
