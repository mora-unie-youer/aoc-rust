use aoc_2020::*;

const DAY: i32 = 6;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|&ch| ch != '\n')
                .fold(0usize, |acc, v| acc | (1 << (v as u8 - b'a')))
                .count_ones() as usize
        })
        .sum();

    let solution2: Solution = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| {
                    person
                        .chars()
                        .fold(0usize, |acc, v| acc | (1 << (v as u8 - b'a')))
                })
                .fold((1 << 26) - 1, |acc, v| acc & v)
                .count_ones() as usize
        })
        .sum();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
