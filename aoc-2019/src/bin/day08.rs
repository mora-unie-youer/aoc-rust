#![feature(iter_array_chunks)]

use aoc_2019::*;
use itertools::Itertools;

const DAY: i32 = 8;
type Solution = String;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
fn main() {
    let input = get_input_text(DAY);
    let input = input.trim();

    let layers: Vec<Vec<usize>> = input
        .chars()
        .array_chunks()
        .map(|v: [char; WIDTH * HEIGHT]| {
            v.into_iter()
                .map(|ch| ch.to_digit(10).unwrap() as _)
                .collect()
        })
        .collect();

    let solution1: Solution = {
        let lowest_layer = layers
            .iter()
            .min_by_key(|layer| layer.iter().filter(|&&v| v == 0).count())
            .unwrap();
        let ones = lowest_layer.iter().filter(|&&v| v == 1).count();
        let twos = lowest_layer.iter().filter(|&&v| v == 2).count();
        (ones * twos).to_string()
    };

    let solution2: Solution = {
        let mut image = [[' '; WIDTH]; HEIGHT];

        for (y, row) in image.iter_mut().enumerate() {
            'pixel: for (x, pixel) in row.iter_mut().enumerate() {
                let i = y * WIDTH + x;
                for layer in &layers {
                    let value = layer[i];
                    if value != 2 {
                        *pixel = if value == 0 { '.' } else { '#' };
                        continue 'pixel;
                    }
                }
            }
        }

        let image = image
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n");
        format!("\n{}", image)
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
