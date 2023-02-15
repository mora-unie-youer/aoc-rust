use std::{cmp::Ordering, iter::Sum, ops::Add};

use aoc_2015::*;

const DAY: i32 = 17;
type Solution = usize;

#[derive(Eq)]
struct CombinationsResult {
    all_count: Solution,
    min_length: Solution,
    min_count: Solution,
}

impl CombinationsResult {
    fn new(min_length: Solution) -> Self {
        Self {
            all_count: 1,
            min_length,
            min_count: 1,
        }
    }
}

impl Default for CombinationsResult {
    fn default() -> Self {
        Self {
            all_count: 0,
            min_length: std::usize::MAX,
            min_count: 0,
        }
    }
}

impl PartialEq for CombinationsResult {
    fn eq(&self, other: &Self) -> bool {
        self.min_length == other.min_length
    }
}

impl PartialOrd for CombinationsResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CombinationsResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.min_length.cmp(&other.min_length)
    }
}

impl Add for CombinationsResult {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let all_count = self.all_count + rhs.all_count;
        match self.cmp(&rhs) {
            Ordering::Less => CombinationsResult {
                all_count,
                min_length: self.min_length,
                min_count: self.min_count,
            },
            Ordering::Greater => CombinationsResult {
                all_count,
                min_length: rhs.min_length,
                min_count: rhs.min_count,
            },
            Ordering::Equal => CombinationsResult {
                all_count,
                min_length: self.min_length,
                min_count: self.min_count + rhs.min_count,
            },
        }
    }
}

impl Sum for CombinationsResult {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(CombinationsResult::default(), |acc, result| acc + result)
    }
}

fn combinations(
    max_sum: usize,
    numbers: &[Solution],
    sum: usize,
    len: usize,
) -> CombinationsResult {
    match sum.cmp(&max_sum) {
        Ordering::Equal => CombinationsResult::new(len),
        Ordering::Greater => CombinationsResult::default(),
        Ordering::Less => numbers
            .iter()
            .enumerate()
            .map(|(i, number)| combinations(max_sum, &numbers[i + 1..], sum + number, len + 1))
            .sum(),
    }
}

const MAX_SUM: usize = 150;
fn main() {
    let input = get_input_text(DAY);
    let numbers: Vec<Solution> = input.lines().map(|line| line.parse().unwrap()).collect();

    let result = combinations(MAX_SUM, &numbers, 0, 0);
    let solution1: Solution = result.all_count;
    let solution2: Solution = result.min_count;

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{combinations, CombinationsResult};

    #[test]
    fn test_combinations_result() {
        let result1 = CombinationsResult {
            all_count: 2,
            min_length: 3,
            min_count: 2,
        };
        let result2 = CombinationsResult {
            all_count: 5,
            min_length: 2,
            min_count: 3,
        };
        assert_eq!(result1.cmp(&result2), Ordering::Greater);

        let result = result1 + result2;
        assert_eq!(result.all_count, 7);
        assert_eq!(result.min_length, 2);
        assert_eq!(result.min_count, 3);
    }

    #[test]
    fn test_combinations() {
        let numbers = [5, 5, 10, 15, 20];
        let result = combinations(25, &numbers, 0, 0);
        assert_eq!(result.all_count, 4);
        assert_eq!(result.min_length, 2);
        assert_eq!(result.min_count, 3);
    }
}
