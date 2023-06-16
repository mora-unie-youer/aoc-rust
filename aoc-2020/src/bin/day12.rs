use aoc_2020::*;

const DAY: i32 = 12;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let commands: Vec<(char, isize)> = input
        .lines()
        .map(|line| {
            let command = line.chars().next().unwrap();
            let repeat = line[1..].parse().unwrap();
            (command, repeat)
        })
        .collect();

    let solution1: Solution = {
        let (mut x, mut y) = (0, 0);
        let (mut dx, mut dy) = (1, 0);
        for &(command, repeat) in &commands {
            match command {
                'N' => y += repeat,
                'S' => y -= repeat,
                'E' => x += repeat,
                'W' => x -= repeat,
                'F' => {
                    x += dx * repeat;
                    y += dy * repeat;
                }
                'R' => {
                    assert!(repeat % 90 == 0);
                    for _ in 0..repeat / 90 {
                        let temp = dx;
                        dx = dy;
                        dy = -temp;
                    }
                }
                'L' => {
                    assert!(repeat % 90 == 0);
                    for _ in 0..repeat / 90 {
                        let temp = dx;
                        dx = -dy;
                        dy = temp;
                    }
                }
                _ => unreachable!(),
            }
        }

        x.unsigned_abs() + y.unsigned_abs()
    };

    let solution2: Solution = {
        let (mut x, mut y) = (0, 0);
        let (mut dx, mut dy) = (10, 1);
        for &(command, repeat) in &commands {
            match command {
                'N' => dy += repeat,
                'S' => dy -= repeat,
                'E' => dx += repeat,
                'W' => dx -= repeat,
                'F' => {
                    x += dx * repeat;
                    y += dy * repeat;
                }
                'R' => {
                    assert!(repeat % 90 == 0);
                    for _ in 0..repeat / 90 {
                        let temp = dx;
                        dx = dy;
                        dy = -temp;
                    }
                }
                'L' => {
                    assert!(repeat % 90 == 0);
                    for _ in 0..repeat / 90 {
                        let temp = dx;
                        dx = -dy;
                        dy = temp;
                    }
                }
                _ => unreachable!(),
            }
        }

        x.unsigned_abs() + y.unsigned_abs()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
