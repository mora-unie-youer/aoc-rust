use aoc_2020::*;

const DAY: i32 = 25;
type Solution = usize;

fn calculate_loop_size(public_key: usize) -> usize {
    let subject_number = 7;
    let mut value = 1;
    let mut loop_size = 0;

    while value != public_key {
        value = (value * subject_number) % 20201227;
        loop_size += 1;
    }

    loop_size
}

fn calculate_encryption_key(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * subject_number) % 20201227;
    }
    value
}

fn main() {
    let input = get_input_text(DAY);
    let (card_pub, door_pub) = input.trim().split_once('\n').unwrap();
    let (card_pub, door_pub) = (card_pub.parse().unwrap(), door_pub.parse().unwrap());

    let _card_loop_size = calculate_loop_size(card_pub);
    let door_loop_size = calculate_loop_size(door_pub);

    let solution1: Solution = calculate_encryption_key(card_pub, door_loop_size);
    show_solution(DAY, solution1);
}
