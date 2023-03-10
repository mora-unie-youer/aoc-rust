#![feature(once_cell)]

use aoc_2016::*;

const DAY: i32 = 15;
type Solution = usize;

#[derive(Debug)]
struct Disc {
    pos: usize,
    size: usize,
}

impl From<&str> for Disc {
    fn from(line: &str) -> Self {
        let regex = regex!(r"has (\d+) positions; at time=0, it is at position (\d+).");
        let captures = regex.captures(line).unwrap();

        Self {
            pos: captures[2].parse().unwrap(),
            size: captures[1].parse().unwrap(),
        }
    }
}

fn solve(discs: &[Disc]) -> Solution {
    (0..)
        .find(|&time| {
            discs
                .iter()
                .enumerate()
                // Each disc rotates while capsule is falling
                .all(|(i, disc)| (disc.pos + time + i + 1) % disc.size == 0)
        })
        .unwrap()
}

fn main() {
    let input = get_input_text(DAY);
    let mut discs: Vec<_> = input.lines().map(Disc::from).collect();

    let solution1: Solution = solve(&discs);
    discs.push(Disc { pos: 0, size: 11 });
    let solution2: Solution = solve(&discs);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::{Disc, solve};

    #[test]
    fn test_solution() {
        let input = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";
        let discs: Vec<_> = input.lines().map(Disc::from).collect();
        assert_eq!(solve(&discs), 5);
    }
}
