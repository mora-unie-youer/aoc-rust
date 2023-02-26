#![feature(array_chunks)]

use aoc_2016::*;

const DAY: i32 = 3;
type Solution = usize;

fn check_triangle(mut sides: [usize; 3]) -> bool {
    sides.sort();
    sides[0] + sides[1] > sides[2]
}

fn main() {
    let input = get_input_text(DAY);
    let sides: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect();

    let solution1: Solution = sides
        .iter()
        .filter(|&s| check_triangle([s[0], s[1], s[2]]))
        .count();

    let solution2: Solution = sides
        .array_chunks()
        .map(|[s1, s2, s3]| {
            s1.iter()
                .zip(s2.iter().zip(s3.iter()))
                .filter(|&(&a, (&b, &c))| check_triangle([a, b, c]))
                .count()
        })
        .sum();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
