use aoc_2024::*;

const DAY: i32 = 3;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = {
        let regex = regex!(r"mul\((\d{1,3}),(\d{1,3})\)");
        regex
            .captures_iter(&input)
            .map(|capture| {
                let a = capture[1].parse::<Solution>().unwrap();
                let b = capture[2].parse::<Solution>().unwrap();
                a * b
            })
            .sum()
    };

    let solution2: Solution = {
        let regex = regex!(r"(mul|do|don't)(\((\d{1,3}),(\d{1,3})\)|\(\))");
        regex
            .captures_iter(&input)
            .fold((0, true), |(sum, enabled), capture| match &capture[1] {
                "do" => (sum, true),
                "don't" => (sum, false),

                "mul" => {
                    let a = capture[3].parse::<Solution>().unwrap();
                    let b = capture[4].parse::<Solution>().unwrap();
                    (sum + a * b * (enabled as usize), enabled)
                }

                _ => unreachable!(),
            })
            .0
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
