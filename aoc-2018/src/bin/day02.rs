use aoc_2018::*;

const DAY: i32 = 2;
type Solution = String;

fn main() {
    let input = get_input_text(DAY);
    let boxes: Vec<_> = input.lines().collect();

    let solution1: Solution = {
        let (mut twos, mut threes) = (0, 0);
        for id in &boxes {
            let mut counts = [0; 26];
            for b in id.bytes() {
                counts[(b - b'a') as usize] += 1;
            }

            let (mut has_two, mut has_three) = (false, false);
            for &c in counts.iter() {
                if c == 2 {
                    has_two = true;
                }
                if c == 3 {
                    has_three = true;
                }
            }

            twos += has_two as usize;
            threes += has_three as usize;
        }

        (twos * threes).to_string()
    };

    let solution2: Solution = {
        let mut result = String::new();

        'main: for i in 0..boxes.len() {
            for j in i + 1..boxes.len() {
                let (a, b) = (boxes[i], boxes[j]);
                let zip_iter = a.chars().zip(b.chars());
                let diffs = zip_iter.clone().filter(|(a, b)| a != b).count();
                if diffs == 1 {
                    result = zip_iter.filter(|(a, b)| a == b).map(|(a, _)| a).collect();
                    break 'main;
                }
            }
        }

        result
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
