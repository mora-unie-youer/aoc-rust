use aoc_2022::*;

const DAY: i32 = 6;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input
        .as_bytes()
        .windows(4)
        .position(|chunk| (1..chunk.len()).all(|i| !chunk[i..].contains(&chunk[i - 1])))
        .unwrap()
        + 4;

    let solution2: Solution = input
        .as_bytes()
        .windows(14)
        .position(|chunk| (1..chunk.len()).all(|i| !chunk[i..].contains(&chunk[i - 1])))
        .unwrap()
        + 14;

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
