use aoc_2016::*;

const DAY: i32 = 20;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let mut blocked_ranges: Vec<(u32, u32)> = input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse::<u32>().unwrap(), end.parse::<u32>().unwrap())
        })
        .collect();
    blocked_ranges.sort();

    let mut merged_ranges = vec![blocked_ranges[0]];
    for (start, end) in blocked_ranges.into_iter().skip(1) {
        if start <= merged_ranges.last().unwrap().1 {
            let last = merged_ranges.last_mut().unwrap();
            last.1 = std::cmp::max(last.1, end);
        } else {
            merged_ranges.push((start, end));
        }
    }

    let mut allowed_ips = vec![];
    let mut last_end = 0;
    for (start, end) in merged_ranges.into_iter() {
        allowed_ips.extend(last_end..start as usize);
        last_end = end as usize + 1;
    }
    allowed_ips.extend(last_end..=std::u32::MAX as usize);
    
    let solution1: Solution = allowed_ips[0];
    let solution2: Solution = allowed_ips.len();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
