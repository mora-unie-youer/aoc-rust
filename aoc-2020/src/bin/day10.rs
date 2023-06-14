use aoc_2020::*;

const DAY: i32 = 10;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let mut joltages: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    joltages.sort();
    joltages.push(joltages.last().unwrap() + 3);

    let solution1: Solution = {
        let joltages = joltages.clone();

        let counts = joltages
            .iter()
            .fold((0, [0, 0, 0]), |(prev, mut acc), &v| {
                let diff = v - prev - 1;
                acc[diff] += 1;
                (v, acc)
            })
            .1;

        counts[0] * counts[2]
    };

    let solution2: Solution = {
        let joltages = {
            let mut v = vec![0];
            v.extend(joltages);
            v
        };

        let mut arrangements = vec![0; joltages.len()];
        arrangements[0] = 1;

        for i in 0..joltages.len() {
            for j in i + 1..joltages.len().min(i + 4) {
                if joltages[j] - joltages[i] <= 3 {
                    arrangements[j] += arrangements[i];
                }
            }
        }

        *arrangements.last().unwrap()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
