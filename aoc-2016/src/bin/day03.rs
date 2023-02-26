#![feature(iter_array_chunks)]
#![feature(once_cell)]

use aoc_2016::*;

const DAY: i32 = 3;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let regex = regex!(r"\s*(\d+)\s*(\d+)\s*(\d+)");

    let solution1: Solution = input
        .lines()
        .filter(|line| {
            let mut result: Vec<_> = regex
                .captures(line)
                .unwrap()
                .iter()
                .skip(1)
                .map(|capture| capture.unwrap().as_str().parse::<usize>().unwrap())
                .collect();
            result.sort();
            result[0] + result[1] > result[2]
        })
        .count();

    let solution2: Solution = input
        .lines()
        .array_chunks()
        .map(|lines: [&str; 3]| {
            let captures: Vec<Vec<usize>> = lines
                .iter()
                .map(|line| regex.captures(line).unwrap())
                .map(|captures| {
                    captures
                        .iter()
                        .skip(1)
                        .map(|capture| capture.unwrap().as_str().parse().unwrap())
                        .collect()
                })
                .collect();

            captures[0]
                .iter()
                .zip(captures[1].iter().zip(captures[2].iter()))
                .filter(|&(&a, (&b, &c))| {
                    let mut sides: [usize; 3] = [a, b, c];
                    sides.sort();
                    sides[0] + sides[1] > sides[2]
                })
                .count()
        })
        .sum();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
