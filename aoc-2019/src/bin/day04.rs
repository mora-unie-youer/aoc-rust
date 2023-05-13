use aoc_2019::*;

const DAY: i32 = 4;
type Solution = usize;

fn part1(num: &i32) -> bool {
    let mut num = *num;
    let mut last_digit = num % 10;
    let mut has_double = false;
    num /= 10;

    while num > 0 {
        let digit = num % 10;
        if digit > last_digit {
            return false;
        }

        if digit == last_digit {
            has_double = true;
        }

        last_digit = digit;
        num /= 10;
    }

    has_double
}

fn part2(num: &i32) -> bool {
    let mut num = *num;
    let mut seq_count = 1;
    let mut last_digit = num % 10;
    let mut has_double = false;
    num /= 10;

    while num > 0 {
        let digit = num % 10;
        if digit > last_digit {
            return false;
        }

        if digit == last_digit {
            seq_count += 1;
        } else {
            if seq_count == 2 {
                has_double = true;
            }

            seq_count = 1;
        }

        last_digit = digit;
        num /= 10;
    }

    has_double || seq_count == 2
}

fn main() {
    let input = get_input_text(DAY);
    let (low, high) = input.trim().split_once('-').unwrap();
    let (low, high) = (low.parse().unwrap(), high.parse().unwrap());

    let solution1: Solution = (low..=high).filter(part1).count();
    let solution2: Solution = (low..=high).filter(part2).count();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        assert!(part1(&111111));
        assert!(!part1(&223450));
        assert!(!part1(&123789));
    }

    #[test]
    fn test_part2() {
        assert!(part2(&112233));
        assert!(!part2(&123444));
        assert!(part2(&111122));
    }
}
