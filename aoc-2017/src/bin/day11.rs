use std::collections::HashSet;

use aoc_2017::*;

const DAY: i32 = 11;
type Solution = usize;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Pos(isize, isize);
impl Pos {
    fn distance(&self) -> Solution {
        let x = self.0;
        let y = self.1;
        let z = -x - y;
        x.abs().max(y.abs()).max(z.abs()) as usize
    }

    fn step(&mut self, direction: &str) {
        match direction {
            "n" => self.1 += 1,
            "nw" => {
                self.0 -= 1;
                self.1 += 1;
            }
            "ne" => self.0 += 1,
            "s" => self.1 -= 1,
            "sw" => self.0 -= 1,
            "se" => {
                self.0 += 1;
                self.1 -= 1;
            }
            _ => unreachable!(),
        }
    }
}

fn solve_part1(input: &str) -> Solution {
    let mut pos = Pos::default();
    input.trim().split(',').for_each(|dir| pos.step(dir));
    pos.distance()
}

fn solve_part2(input: &str) -> Solution {
    let mut pos = Pos::default();
    let mut visited = HashSet::new();

    input.trim().split(',').for_each(|dir| {
        pos.step(dir);
        visited.insert(pos);
    });

    visited
        .into_iter()
        .map(|pos| pos.distance())
        .max()
        .unwrap()

}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = solve_part1(&input);
    let solution2: Solution = solve_part2(&input);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::solve_part1;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1("ne,ne,ne"), 3);
        assert_eq!(solve_part1("ne,ne,sw,sw"), 0);
        assert_eq!(solve_part1("ne,ne,s,s"), 2);
        assert_eq!(solve_part1("se,sw,se,sw,sw"), 3);
    }
}
