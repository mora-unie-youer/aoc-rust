use aoc_2020::*;

const DAY: i32 = 1;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let report: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    let solution1: Solution = {
        let mut result = 0;
        'main: for (i, a) in report.iter().enumerate() {
            for b in report.iter().skip(i + 1) {
                if a + b == 2020 {
                    result = a * b;
                    break 'main;
                }
            }
        }

        result
    };

    let solution2: Solution = {
        let mut result = 0;
        'main: for (i, a) in report.iter().enumerate() {
            for (j, b) in report.iter().enumerate().skip(i + 1) {
                for c in report.iter().skip(j + 1) {
                    if a + b + c == 2020 {
                        result = a * b * c;
                        break 'main;
                    }
                }
            }
        }

        result
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
