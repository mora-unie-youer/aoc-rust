use aoc_2020::*;

const DAY: i32 = 15;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let numbers: Vec<usize> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let solution1: Solution = {
        let mut numbers = numbers.clone();

        while numbers.len() < 2020 {
            let last = numbers.last().unwrap();
            let found = numbers
                .iter()
                .rev()
                .enumerate()
                .skip(1)
                .find(|&(_, v)| v == last);

            match found {
                Some((i, _)) => numbers.push(i),
                None => numbers.push(0),
            }
        }

        *numbers.last().unwrap()
    };

    // Max value can be 30_000_000 - 1, so that's why
    // It's not needed, but I don't care
    const LIMIT: usize = 30_000_000;
    let solution2: Solution = {
        let mut visited = vec![0usize; LIMIT];
        numbers
            .iter()
            .enumerate()
            .take(numbers.len() - 1)
            .for_each(|(i, &v)| visited[v] = i + 1);

        let mut last_num = *numbers.last().unwrap();
        for i in numbers.len() + 1..=LIMIT {
            let mut was_visited = i - 1;
            std::mem::swap(&mut was_visited, &mut visited[last_num]);

            last_num = if was_visited == 0 {
                0
            } else {
                i - was_visited - 1
            };
        }

        last_num
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
