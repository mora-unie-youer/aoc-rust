use aoc_2024::*;

const DAY: i32 = 13;
type Solution = usize;

fn solve_task(task: &str, offset: f64) -> Option<usize> {
    // c = n*a + m*b
    //
    // c_x = n*a_x + m*b_x
    // c_y = n*a_y + m*b_y
    //
    // c_x/a_x = n + m*b_x/a_x
    // c_y = (c_x/a_x - m*b_x/a_x) * a_y + m*b_y
    // c_y*a_x/a_y = c_x - m*b_x + m*b_y*a_x/a_y
    //
    // m = (c_y*a_x/a_y - c_x) / (b_y*a_x/a_y - b_x)
    // m = (c_y*ax - c_x*a_y) / (b_y*a_x - b_x*a_y)
    // n = (c_x - m*b_x) / a_x

    let mut lines = task.lines();
    let button_a = lines.next().unwrap();
    let button_b = lines.next().unwrap();
    let prize_coords = lines.next().unwrap();

    let mut button_a = button_a.split([',', '+']);
    let ax = button_a.nth(1).unwrap();
    let ay = button_a.nth(1).unwrap();
    let a: (f64, f64) = (ax.parse().unwrap(), ay.parse().unwrap());

    let mut button_b = button_b.split([',', '+']);
    let bx = button_b.nth(1).unwrap();
    let by = button_b.nth(1).unwrap();
    let b: (f64, f64) = (bx.parse().unwrap(), by.parse().unwrap());

    let mut prize_coords = prize_coords.split([',', '=']);
    let cx = prize_coords.nth(1).unwrap();
    let cy = prize_coords.nth(1).unwrap();
    let c: (f64, f64) = (
        cx.parse::<f64>().unwrap() + offset,
        cy.parse::<f64>().unwrap() + offset,
    );

    if a.1 == a.0 * b.1 / b.0 {
        // Vectors are parallel
        // NOTE: there's one of the case, when it has correct solution
        dbg!("Here");
        return None;
    }

    let m = (a.0 * c.1 - a.1 * c.0) / (a.0 * b.1 - a.1 * b.0);
    let n = (c.0 - m * b.0) / a.0;

    if n.is_sign_negative() || m.is_sign_negative() {
        // We can't "unpress" buttons
        return None;
    }

    // dbg!(a, b, c, m, n);
    (m.fract() == 0.0 && n.fract() == 0.0).then_some(n as usize * 3 + m as usize)
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input
        .trim()
        .split("\n\n")
        .filter_map(|task| solve_task(task, 0.0))
        .sum();
    let solution2: Solution = input
        .trim()
        .split("\n\n")
        .filter_map(|task| solve_task(task, 10000000000000.0))
        .sum();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
