use aoc_2021::*;

const DAY: i32 = 2;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, amount)| (dir, amount.parse::<usize>().unwrap()))
        .fold([0, 0], |[x, z], (dir, amount)| match dir {
            "forward" => [x + amount, z],
            "down" => [x, z + amount],
            "up" => [x, z - amount],
            _ => unreachable!(),
        })
        .iter()
        .product();

    let solution2: Solution = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, amount)| (dir, amount.parse::<usize>().unwrap()))
        .fold([0, 0, 0], |[x, z, aim], (dir, amount)| match dir {
            "forward" => [x + amount, z + amount * aim, aim],
            "down" => [x, z, aim + amount],
            "up" => [x, z, aim - amount],
            _ => unreachable!(),
        })
        .iter()
        .take(2)
        .product();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
