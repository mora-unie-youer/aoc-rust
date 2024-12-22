use aoc_2024::*;
use itertools::{Itertools, iterate};

const DAY: i32 = 22;
type Solution = usize;

fn next_number(n: &usize) -> usize {
    const MASK: usize = (1 << 24) - 1;

    let mut n = (n ^ (n << 6)) & MASK;
    n ^= n >> 5;
    (n ^ (n << 11)) & MASK
}

fn index(a: isize, b: isize, c: isize, d: isize) -> usize {
    (6859 * (a + 9) + 361 * (b + 9) + 19 * (c + 9) + d + 9) as usize
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input
        .lines()
        .map(|line| line.parse().unwrap())
        .flat_map(|v| iterate(v, next_number).nth(2000))
        .sum();

    let solution2: Solution = {
        let mut sequences = vec![0; 19usize.pow(4)];
        let mut used = vec![0; 19usize.pow(4)];

        for (line, i) in input.lines().zip(1..) {
            for (a, b, c, d, e) in iterate(line.parse().unwrap(), next_number)
                .take(2001)
                .map(|v| (v % 10) as isize)
                .tuple_windows()
            {
                let j = index(b - a, c - b, d - c, e - d);
                if used[j] != i {
                    used[j] = i;
                    sequences[j] += e as usize;
                }
            }
        }

        sequences.into_iter().max().unwrap()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
