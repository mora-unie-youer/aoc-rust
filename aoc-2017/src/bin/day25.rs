use std::collections::HashMap;

use aoc_2017::*;

const DAY: i32 = 25;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let (start, states) = input.split_once("\n\n").unwrap();

    let (start_state, steps) = {
        let parts: Vec<_> = start.split_ascii_whitespace().collect();
        let state = parts[3].chars().next().unwrap();
        let steps = parts[9].parse::<usize>().unwrap();
        (state, steps)
    };

    let states = {
        let mut map: HashMap<(char, bool), (bool, isize, char)> = HashMap::new();

        for state in states.split("\n\n") {
            // Get rid of many . chars
            let state: String = state.chars().filter(|&ch| ch != '.').collect();
            let parts: Vec<_> = state.split_ascii_whitespace().collect();

            let name = parts[2].chars().next().unwrap();
            let (false_value, true_value) = (parts[13] == "1", parts[36] == "1");
            let (false_dir, true_dir) = (parts[20], parts[43]);
            let false_dir = if false_dir == "right" { 1 } else { -1 };
            let true_dir = if true_dir == "right" { 1 } else { -1 };

            let (false_next, true_next) = (
                parts[25].chars().next().unwrap(),
                parts[48].chars().next().unwrap(),
            );

            map.insert((name, false), (false_value, false_dir, false_next));
            map.insert((name, true), (true_value, true_dir, true_next));
        }

        map
    };

    let solution: Solution = {
        let mut tape: HashMap<isize, bool> = HashMap::new();
        let mut position = 0;
        let mut state = start_state;

        for _ in 0..steps {
            let value = tape.entry(position).or_insert(false);
            let new_state = states[&(state, *value)];

            *value = new_state.0;
            position += new_state.1;
            state = new_state.2;
        }

        tape.values().filter(|&&v| v).count()
    };

    show_solution(DAY, solution);
}
