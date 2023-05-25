use aoc_2019::*;

const DAY: i32 = 16;
type Solution = isize;

fn pattern_element(n: usize, offset: usize) -> isize {
    let v = (offset + 1) % (4 * n);

    if v < n {
        0
    } else if v < 2 * n {
        1
    } else if v < 3 * n {
        0
    } else {
        -1
    }
}

fn fft(number: &[isize]) -> Vec<isize> {
    let mut result = vec![];

    for n in 0..number.len() {
        let digit: isize = number
            .iter()
            .enumerate()
            .map(|(offset, v)| v * pattern_element(n + 1, offset))
            .sum();
        result.push(digit.abs() % 10);
    }

    result
}

fn improved_fft(mut number: Vec<isize>) -> Vec<isize> {
    let mut sums = vec![];
    let mut total = 0;
    sums.push(0);

    for v in &number {
        total += v;
        sums.push(total);
    }

    for (i, digit) in number.iter_mut().enumerate() {
        let value = sums.last().unwrap() - sums[i];
        *digit = value % 10;
    }

    number
}

fn main() {
    let input = get_input_text(DAY);

    let number: Vec<_> = input
        .trim()
        .chars()
        .map(|v| v.to_digit(10).unwrap() as isize)
        .collect();

    let solution1: Solution = {
        let new_number = (0..100).fold(number.clone(), |acc, _| fft(&acc));
        new_number
            .iter()
            .take(8)
            .fold(0, |acc, digit| acc * 10 + digit)
    };

    let solution2: Solution = {
        let count = number.len();
        let offset = number.iter().take(7).fold(0, |acc, digit| acc * 10 + digit) as usize;
        let new_count = count * 10_000 - offset;
        let number: Vec<_> = number
            .into_iter()
            .cycle()
            .skip(offset)
            .take(new_count)
            .collect();

        let new_number = (0..100).fold(number, |acc, _| improved_fft(acc));
        new_number
            .iter()
            .take(8)
            .fold(0, |acc, digit| acc * 10 + digit)
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
