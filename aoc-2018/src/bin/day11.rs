use aoc_2018::*;

const DAY: i32 = 11;
type Solution = String;

fn power_value(input: usize, x: usize, y: usize) -> isize {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += input;
    power *= rack_id;
    ((power / 100) % 10) as isize - 5
}

fn main() {
    let input = get_input_text(DAY);
    let input = input.trim().parse().unwrap();

    let map_size = 300;
    // This map is summed-area table
    let map: Vec<Vec<isize>> = {
        let mut map = vec![vec![0; map_size + 1]; map_size + 1];
        for y in 1..=map_size {
            for x in 1..=map_size {
                let power = power_value(input, x, y);
                map[y][x] = power + map[y - 1][x] + map[y][x - 1] - map[y - 1][x - 1];
            }
        }

        map
    };

    let solution1: Solution = {
        let mut max_power = std::isize::MIN;
        let mut point = (0, 0);
        for y in 1..=map_size - 2 {
            for x in 1..=map_size - 2 {
                let power =
                    map[y + 2][x + 2] - map[y + 2][x - 1] - map[y - 1][x + 2] + map[y - 1][x - 1];
                if power > max_power {
                    max_power = power;
                    point = (x, y);
                }
            }
        }

        format!("{},{}", point.0, point.1)
    };

    let solution2: Solution = {
        let mut max_power = std::isize::MIN;
        let mut result = (0, 0, 0);

        // This is size - 1
        for size in 0..map_size {
            for y in 1..=map_size - size {
                for x in 1..=map_size - size {
                    let power =
                        map[y + size][x + size] - map[y + size][x - 1] - map[y - 1][x + size]
                            + map[y - 1][x - 1];
                    if power > max_power {
                        max_power = power;
                        result = (x, y, size + 1);
                    }
                }
            }
        }

        format!("{},{},{}", result.0, result.1, result.2)
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::power_value;

    #[test]
    fn test_power_value() {
        assert_eq!(power_value(8, 3, 5), 4);
        assert_eq!(power_value(57, 122, 79), -5);
        assert_eq!(power_value(39, 217, 196), 0);
        assert_eq!(power_value(71, 101, 153), 4);
    }
}
