use aoc_2018::*;

const DAY: i32 = 14;
type Solution = String;

fn solve_part1(input: usize) -> Solution {
    let mut table = vec![3, 7];
    let (mut elf1, mut elf2) = (0, 1);

    while table.len() < input + 10 {
        let (score1, score2) = (table[elf1], table[elf2]);
        let score = score1 + score2;
        score
            .to_string()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as usize)
            .for_each(|v| table.push(v));

        elf1 = (elf1 + 1 + score1) % table.len();
        elf2 = (elf2 + 1 + score2) % table.len();
    }

    table[input..].iter().map(|v| v.to_string()).collect()
}

fn solve_part2(input: usize) -> Solution {
    let input: Vec<_> = input
        .to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect();

    let mut table = vec![3, 7];
    let (mut elf1, mut elf2) = (0, 1);

    let mut skip_size = 0;
    loop {
        let slice = &table[skip_size..];
        if let Some(i) = slice
            .windows(input.len())
            .position(|window| window == input)
        {
            let pos = skip_size + i;
            return pos.to_string();
        }

        // Need to skip talbe if there are more than 2 * input.len() elements on table
        skip_size = 0.max(table.len() as isize - 2 * input.len() as isize) as usize;

        let (score1, score2) = (table[elf1], table[elf2]);
        let score = score1 + score2;
        score
            .to_string()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as usize)
            .for_each(|v| table.push(v));

        elf1 = (elf1 + 1 + score1) % table.len();
        elf2 = (elf2 + 1 + score2) % table.len();
    }
}

fn main() {
    let input = get_input_text(DAY);
    let input = input.trim().parse().unwrap();

    let solution1: Solution = solve_part1(input);
    let solution2: Solution = solve_part2(input);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
