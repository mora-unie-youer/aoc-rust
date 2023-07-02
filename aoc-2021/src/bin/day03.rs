use aoc_2021::*;

const DAY: i32 = 3;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let data: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch == '1').collect())
        .collect();
    let entry_count = data.len();
    let entry_length = data[0].len();

    let solution1: Solution = {
        let mut digits = vec![0; entry_length];
        for entry in &data {
            for (i, &bit) in entry.iter().enumerate() {
                digits[i] += bit as usize;
            }
        }

        let bit_mask = (1 << entry_length) - 1;
        let gamma = digits
            .iter()
            .map(|&v| (v > entry_count / 2) as usize)
            .fold(0, |acc, v| (acc << 1) | v);
        let epsilon = !gamma & bit_mask;

        gamma * epsilon
    };

    let solution2: Solution = {
        let mut oxygen_data = data.clone();
        let mut carbon_data = data;

        let mut bit = 0;
        while oxygen_data.len() != 1 {
            let mut count = 0;
            for entry in &oxygen_data {
                count += entry[bit] as usize;
            }

            let filter = oxygen_data.len() as isize - count as isize * 2 <= 0;
            oxygen_data.retain(|entry| entry[bit] == filter);

            bit += 1;
        }

        let mut bit = 0;
        while carbon_data.len() != 1 {
            let mut count = 0;
            for entry in &carbon_data {
                count += entry[bit] as usize;
            }

            let filter = carbon_data.len() as isize - count as isize * 2 > 0;
            carbon_data.retain(|entry| entry[bit] == filter);

            bit += 1;
        }

        let oxygen = oxygen_data[0]
            .iter()
            .fold(0, |acc, &v| (acc << 1) | v as usize);
        let carbon = carbon_data[0]
            .iter()
            .fold(0, |acc, &v| (acc << 1) | v as usize);

        oxygen * carbon
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
