use aoc_2016::*;

const DAY: i32 = 6;
type Solution = String;

fn main() {
    let input = get_input_text(DAY);

    let mut frequencies = vec![];
    for i in 0..8 {
        let mut frequency = vec![0usize; 26];
        input
            .chars()
            .skip(i)
            .step_by(9)
            .map(|ch| ch as u8 - b'a')
            .for_each(|ch| frequency[ch as usize] += 1);

        let char_frequency: Vec<_> = frequency
            .into_iter()
            .enumerate()
            .map(|(i, count)| ((i as u8 + b'a') as char, count))
            .collect();
        frequencies.push(char_frequency);
    }

    let solution1: Solution = {
        let mut sorted_frequencies = frequencies.clone();
        sorted_frequencies.iter_mut().for_each(|v| v.sort_by(|a, b| b.1.cmp(&a.1)));
        sorted_frequencies.iter().map(|v| v[0].0).collect()
    };

    let solution2: Solution = {
        let mut sorted_frequencies = frequencies.clone();
        sorted_frequencies.iter_mut().for_each(|v| v.sort_by(|a, b| a.1.cmp(&b.1)));
        sorted_frequencies.iter().map(|v| v[0].0).collect()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
