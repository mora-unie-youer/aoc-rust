use aoc_2020::*;

const DAY: i32 = 5;
type Solution = usize;

fn get_seat_id(seat: &str) -> usize {
    let (row_seat, col_seat) = (&seat[0..7], &seat[7..]);

    let (mut row_low, mut row_high) = (0, 127);
    let (mut col_low, mut col_high) = (0, 7);

    for step in row_seat.chars() {
        let dy = (row_high - row_low) / 2;

        match step {
            'F' => row_high = row_low + dy,
            'B' => row_low += dy + 1,
            _ => unreachable!(),
        }
    }

    for step in col_seat.chars() {
        let dx = (col_high - col_low) / 2;

        match step {
            'L' => col_high = col_low + dx,
            'R' => col_low += dx + 1,
            _ => unreachable!(),
        }
    }

    row_low * 8 + col_low
}

fn main() {
    let input = get_input_text(DAY);
    let mut seats: Vec<_> = input.lines().map(get_seat_id).collect();
    seats.sort();

    let solution1: Solution = *seats.last().unwrap();
    let solution2: Solution = seats.windows(3).find(|v| v[0] + 1 != v[1]).unwrap()[0] + 1;

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
