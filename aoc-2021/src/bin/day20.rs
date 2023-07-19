use std::collections::VecDeque;

use aoc_2021::*;

const DAY: i32 = 20;
type Solution = usize;

fn enhance(
    mut image: VecDeque<VecDeque<bool>>,
    table: &[bool],
    iteration: usize,
) -> VecDeque<VecDeque<bool>> {
    // Find filler to extend with
    let filler = table[0] && iteration % 2 != 0;

    // Extend image columns
    for row in &mut image {
        for _ in 0..3 {
            row.push_front(filler);
            row.push_back(filler);
        }
    }

    // Extend image rows
    let mut empty_row = VecDeque::new();
    empty_row.resize(image[0].len(), filler);
    for _ in 0..3 {
        image.push_front(empty_row.clone());
        image.push_back(empty_row.clone());
    }

    // Enhancing image
    let mut new_image = image.clone();
    for y in 1..new_image.len() - 1 {
        for x in 1..new_image.len() - 1 {
            let enhancement = (-1..=1)
                .flat_map(|dy| {
                    (-1..=1)
                        .map(move |dx| (y as isize + dy, x as isize + dx))
                        .map(|(y, x)| image[y as usize][x as usize])
                })
                .fold(0, |acc, pixel| (acc << 1) | pixel as usize);
            new_image[y][x] = table[enhancement];
        }
    }

    // Shrinking image by 1 pixel from each side
    new_image.pop_front();
    new_image.pop_back();
    for row in &mut new_image {
        row.pop_front();
        row.pop_back();
    }

    new_image
}

fn main() {
    let input = get_input_text(DAY);
    let (table, image) = input.split_once("\n\n").unwrap();

    let table: Vec<bool> = table.chars().map(|ch| ch == '#').collect();
    let image: VecDeque<VecDeque<bool>> = image
        .lines()
        .map(|line| line.chars().map(|ch| ch == '#').collect())
        .collect();

    let solution1: Solution = (0..2)
        .fold(image.clone(), |acc, i| enhance(acc, &table, i))
        .into_iter()
        .flatten()
        .filter(|&pixel| pixel)
        .count();
    let solution2: Solution = (0..50)
        .fold(image, |acc, i| enhance(acc, &table, i))
        .into_iter()
        .flatten()
        .filter(|&pixel| pixel)
        .count();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
