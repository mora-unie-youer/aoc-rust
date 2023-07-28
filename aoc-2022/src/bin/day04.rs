use aoc_2022::*;

const DAY: i32 = 4;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input
        .lines()
        .filter(|line| {
            let nums: Vec<usize> = line.split(['-', ',']).map(|v| v.parse().unwrap()).collect();

            let left = nums[0] <= nums[2] && nums[1] >= nums[3];
            let right = nums[2] <= nums[0] && nums[3] >= nums[1];
            left || right
        })
        .count();

    let solution2: Solution = input
        .lines()
        .filter(|line| {
            let nums: Vec<usize> = line.split(['-', ',']).map(|v| v.parse().unwrap()).collect();
            nums[0] <= nums[3] && nums[2] <= nums[1]
        })
        .count();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
