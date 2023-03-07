use aoc_2016::*;

const DAY: i32 = 9;
type Solution = usize;

fn solve(mut input: &str, recursive: bool) -> Solution {
    let mut length = 0;

    // There's no more repeats
    if !input.contains('(') {
        return input.len();
    }

    while input.contains('(') {
        let marker_start = input.find('(').unwrap();
        let marker_end = input.find(')').unwrap();
        let marker = &input[marker_start + 1..marker_end];

        let (size, count) = marker.split_once('x').unwrap();
        let (size, count): (usize, usize) = (size.parse().unwrap(), count.parse().unwrap());

        length += marker_start;
        if recursive {
            length += count * solve(&input[marker_end + 1..=marker_end + size], recursive);
        } else {
            length += count * size;
        }

        input = &input[marker_end + size + 1..];
    }

    length + input.len()
}

fn main() {
    let input = get_input_text(DAY);
    let input = input.trim();

    let solution1: Solution = solve(input, false);
    let solution2: Solution = solve(input, true);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_part1() {
        assert_eq!(solve("ADVENT", false), 6);
        assert_eq!(solve("A(1x5)BC", false), 7);
        assert_eq!(solve("(3x3)XYZ", false), 9);
        assert_eq!(solve("A(2x2)BCD(2x2)EFG", false), 11);
        assert_eq!(solve("(6x1)(1x3)A", false), 6);
        assert_eq!(solve("X(8x2)(3x3)ABCY", false), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve("(3x3)XYZ", true), 9);
        assert_eq!(solve("X(8x2)(3x3)ABCY", true), 20);
        assert_eq!(solve("(27x12)(20x12)(13x14)(7x10)(1x12)A", true), 241920);
        assert_eq!(
            solve(
                "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
                true
            ),
            445
        );
    }
}
