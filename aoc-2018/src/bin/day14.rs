use aoc_2018::*;

const DAY: i32 = 14;
type Solution = String;

fn iteration(elf1: &mut usize, elf2: &mut usize, table: &mut Vec<usize>) {
    let (score1, score2) = (table[*elf1], table[*elf2]);
    let score = score1 + score2;

    if score >= 10 {
        table.push(1);
    }
    table.push(score % 10);

    *elf1 = (*elf1 + 1 + score1) % table.len();
    *elf2 = (*elf2 + 1 + score2) % table.len();
}

fn solve_part1(input: usize) -> Solution {
    let mut table = vec![3, 7];
    let (mut elf1, mut elf2) = (0, 1);

    while table.len() < input + 10 {
        iteration(&mut elf1, &mut elf2, &mut table);
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
        if let Some(i) = table[skip_size..]
            .windows(input.len())
            .position(|window| window == input)
        {
            let pos = skip_size + i;
            return pos.to_string();
        }

        iteration(&mut elf1, &mut elf2, &mut table);
        // Need to skip table if there are more than input.len() elements on table
        skip_size = 0.max(table.len() as isize - input.len() as isize - 1) as usize;
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
