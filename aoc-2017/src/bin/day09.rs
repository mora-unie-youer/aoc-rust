use aoc_2017::*;

const DAY: i32 = 9;
type Solution = usize;

fn solve(input: &str) -> (Solution, Solution) {
    let mut score = 0;
    let mut count = 0;

    let mut depth = 0;
    let mut garbage = false;
    let mut ignore_next = false;

    for c in input.chars() {
        if ignore_next {
            ignore_next = false;
            continue;
        }

        match c {
            '!' => ignore_next = true,
            '<' if !garbage => garbage = true,
            '<' if garbage => count += 1,
            '>' => garbage = false,

            '{' if !garbage => {
                depth += 1;
                score += depth;
            }
            '{' if garbage => count += 1,

            '}' if !garbage => depth -= 1,
            '}' if garbage => count += 1,
            _ if garbage => count += 1,
            _ => (),
        }
    }

    (score, count)
}

fn main() {
    let input = get_input_text(DAY);

    let (solution1, solution2) = solve(&input);
    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_calculate_score() {
        assert_eq!(solve("{}").0, 1);
        assert_eq!(solve("{{{}}}").0, 6);
        assert_eq!(solve("{{},{}}").0, 5);
        assert_eq!(solve("{{{},{},{{}}}}").0, 16);
        assert_eq!(solve("{<a>,<a>,<a>,<a>}").0, 1);
        assert_eq!(solve("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
        assert_eq!(solve("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
        assert_eq!(solve("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);
    }

    #[test]
    fn test_count_garbage() {
        assert_eq!(solve("<>").1, 0);
        assert_eq!(solve("<random characters>").1, 17);
        assert_eq!(solve("<<<<>").1, 3);
        assert_eq!(solve("<{!>}>").1, 2);
        assert_eq!(solve("<!!>").1, 0);
        assert_eq!(solve("<!!!>>").1, 0);
        assert_eq!(solve("<{o\"i!a,<{i<a>").1, 10);
    }
}
