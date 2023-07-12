use aoc_2021::*;

const DAY: i32 = 13;
type Solution = String;

fn main() {
    let input = get_input_text(DAY);

    let (dots, folds) = input.split_once("\n\n").unwrap();
    let dots: Vec<(usize, usize)> = dots
        .lines()
        .map(|dot| dot.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    let folds: Vec<(char, usize)> = folds
        .lines()
        .filter_map(|fold| fold.rsplit(' ').next())
        .map(|fold| fold.split_once('=').unwrap())
        .map(|(axis, coord)| (axis.chars().next().unwrap(), coord.parse().unwrap()))
        .collect();

    let solution1: Solution = {
        let mut dots = dots.clone();
        let (axis, coord) = folds[0];

        match axis {
            'x' => dots
                .iter_mut()
                .filter(|(x, _)| *x > coord)
                .for_each(|(x, _)| *x = 2 * coord - *x),
            'y' => dots
                .iter_mut()
                .filter(|(_, y)| *y > coord)
                .for_each(|(_, y)| *y = 2 * coord - *y),
            _ => unreachable!(),
        }

        dots.sort();
        dots.dedup();
        dots.len().to_string()
    };

    let solution2: Solution = {
        let mut dots = dots;
        for (axis, coord) in folds {
            match axis {
                'x' => dots
                    .iter_mut()
                    .filter(|(x, _)| *x > coord)
                    .for_each(|(x, _)| *x = 2 * coord - *x),
                'y' => dots
                    .iter_mut()
                    .filter(|(_, y)| *y > coord)
                    .for_each(|(_, y)| *y = 2 * coord - *y),
                _ => unreachable!(),
            }
            dots.sort();
            dots.dedup();
        }

        let max_x = dots.iter().map(|&(x, _)| x).max().unwrap();
        let max_y = dots.iter().map(|&(_, y)| y).max().unwrap();
        let image = (0..=max_y)
            .map(|y| {
                (0..=max_x)
                    .map(|x| if dots.contains(&(x, y)) { '#' } else { '.' })
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!("\n{}", image)
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
