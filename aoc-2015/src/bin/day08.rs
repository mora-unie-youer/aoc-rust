use aoc_2015::*;

const DAY: i32 = 8;
type Solution = usize;

trait UnescapedLength {
    fn len_unescaped(&self) -> usize;
}

impl UnescapedLength for &str {
    fn len_unescaped(&self) -> usize {
        let mut chars = self.chars();
        let mut length = 0;

        while let Some(ch) = chars.next() {
            length += match ch {
                '\\' => match chars.next() {
                    Some('\\') => 1,
                    Some('\"') => 1,
                    Some('x') => {
                        let _ = chars.next().unwrap();
                        let _ = chars.next().unwrap();
                        // I guess I don't have to parse it
                        1
                    }
                    _ => 2,
                },
                _ => 1,
            };
        }

        length
    }
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input.lines().map(|s| s.len() - s.len_unescaped() + 2).sum();
    let solution2: Solution = input
        .lines()
        .map(|s| s.escape_default().count() - s.len() + 2)
        .sum();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
