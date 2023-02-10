use aoc_2015::*;
use json::{object::Object, JsonValue};

const DAY: i32 = 12;
type Solution = isize;

fn has_red(obj: &Object) -> bool {
    obj.iter()
        .any(|(_, v)| v.is_string() && v.as_str().unwrap() == "red")
}

fn solve(root: &JsonValue, ignore_red: bool) -> Solution {
    match root {
        JsonValue::Number(_) => root.as_isize().unwrap(),
        JsonValue::Array(arr) => arr.iter().map(|v| solve(v, ignore_red)).sum(),
        JsonValue::Object(obj) => {
            if ignore_red && has_red(obj) {
                return 0;
            }

            obj.iter().map(|(_, v)| solve(v, ignore_red)).sum()
        }
        _ => 0,
    }
}

fn main() {
    let input = get_input_text(DAY);
    let root = json::parse(&input).unwrap();

    let solution1: Solution = solve(&root, false);
    let solution2: Solution = solve(&root, true);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
