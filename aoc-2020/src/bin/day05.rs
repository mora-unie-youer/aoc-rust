use aoc_2020::*;

const DAY: i32 = 5;
type Solution = usize;

fn get_seat_id(seat: &str) -> usize {
    seat.chars()
        .map(|ch| matches!(ch, 'B' | 'R'))
        .fold(0, |acc, v| (acc << 1) | v as usize)
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
