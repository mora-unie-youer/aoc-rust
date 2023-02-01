use aoc_2015::*;

const DAY: i32 = 2;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input
        .lines()
        .map(|dims| {
            let sides: Vec<Solution> = dims.split('x').map(|v| v.parse().unwrap()).collect();
            let (width, height, length) = (sides[0], sides[1], sides[2]);
            let (area1, area2, area3) = (width * height, width * length, height * length);
            2 * (area1 + area2 + area3) + area1.min(area2.min(area3))
        })
        .sum();
    let solution2: Solution = input
        .lines()
        .map(|dims| {
            let sides: Vec<Solution> = dims.split('x').map(|v| v.parse().unwrap()).collect();
            let (width, height, length) = (sides[0], sides[1], sides[2]);
            2 * (width + height).min((width + length).min(height + length))
                + width * height * length
        })
        .sum();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
