use aoc_2020::*;

const DAY: i32 = 23;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let cups: Vec<_> = input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect();

    let solution1: Solution = {
        const SIZE: usize = 9;
        const MOVES: usize = 100;

        // Mapping value -> value
        let mut mapping = vec![0; SIZE];
        for (i, cup) in cups.iter().enumerate() {
            mapping[cup - 1] = *cups.get(i + 1).or_else(|| cups.first()).unwrap() - 1;
        }

        let mut current = cups[0] - 1;
        for _ in 0..MOVES {
            let a = mapping[current];
            let b = mapping[a];
            let c = mapping[b];

            let dest = mapping[c];
            let real_dest =
                std::iter::successors(Some(current), |v| v.checked_sub(1).or(Some(SIZE - 1)))
                    .skip(1)
                    .find(|&v| v != a && v != b && v != c)
                    .unwrap();

            let temp = mapping[real_dest];
            mapping[current] = dest;
            mapping[real_dest] = a;
            mapping[c] = temp;
            current = dest;
        }

        std::iter::successors(mapping.first(), |&&v| mapping.get(v))
            .take_while(|&&v| v != 0)
            .fold(0, |acc, v| 10 * acc + v + 1)
    };

    let solution2: Solution = {
        const SIZE: usize = 1_000_000;
        const MOVES: usize = 10_000_000;

        // Mapping value -> value
        let mut mapping: Vec<_> = (1..=SIZE).collect();
        for (i, cup) in cups.iter().enumerate() {
            mapping[cup - 1] = *cups.get(i + 1).unwrap_or(&10) - 1;
        }

        let mut current = cups[0] - 1;
        *mapping.last_mut().unwrap() = current;
        for _ in 0..MOVES {
            let a = mapping[current];
            let b = mapping[a];
            let c = mapping[b];

            let dest = mapping[c];
            let real_dest =
                std::iter::successors(Some(current), |v| v.checked_sub(1).or(Some(SIZE - 1)))
                    .skip(1)
                    .find(|&v| v != a && v != b && v != c)
                    .unwrap();

            let temp = mapping[real_dest];
            mapping[current] = dest;
            mapping[real_dest] = a;
            mapping[c] = temp;
            current = dest;
        }

        let a = mapping[0] + 1;
        let b = mapping[a - 1] + 1;
        a * b
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
