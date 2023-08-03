use aoc_2022::*;

const DAY: i32 = 10;
type Solution = String;

fn draw(screen: &mut [char; 240], x: isize, cycle: usize) {
    let covered = (x - 1..=x + 1).contains(&(cycle as isize % 40));
    if covered {
        screen[cycle] = '#';
    }
}

fn main() {
    let input = get_input_text(DAY);

    let mut x = 1;
    let mut cycles = vec![x];

    let mut screen = [' '; 240];
    for op in input.lines() {
        draw(&mut screen, x, cycles.len() - 1);
        cycles.push(x);
        if op.starts_with('a') {
            draw(&mut screen, x, cycles.len() - 1);
            cycles.push(x);

            x += &op[5..].parse().unwrap();
        }
    }

    let solution1: Solution = cycles
        .iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .take(6)
        .fold(0, |value, (i, x)| value + i as isize * x)
        .to_string();

    let solution2: Solution = {
        let screen = screen
            .chunks(40)
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        format!("\n{screen}")
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
