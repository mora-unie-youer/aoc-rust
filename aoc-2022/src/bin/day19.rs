use aoc_2022::*;
use itertools::Itertools;

const DAY: i32 = 19;
type Solution = usize;

type Recipe = [usize; 4];

#[derive(Debug)]
struct Blueprint {
    recipies: [Recipe; 4],
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();

        let ore_robot_price = parts.nth(6).unwrap().parse().unwrap();
        let clay_robot_price = parts.nth(5).unwrap().parse().unwrap();

        let obsidian_ore_price = parts.nth(5).unwrap().parse().unwrap();
        let obsidian_clay_price = parts.nth(2).unwrap().parse().unwrap();

        let geode_ore_price = parts.nth(5).unwrap().parse().unwrap();
        let geode_obsidian_price = parts.nth(2).unwrap().parse().unwrap();

        let recipies = [
            // Ore robot recipe
            [ore_robot_price, 0, 0, 0],
            // Clay robot recipe
            [clay_robot_price, 0, 0, 0],
            // Obisidian robot recipe
            [obsidian_ore_price, obsidian_clay_price, 0, 0],
            // Geode robot recipe
            [geode_ore_price, 0, geode_obsidian_price, 0],
        ];

        Self { recipies }
    }
}

fn simulate_blueprint(blueprint: &Blueprint, max_time: usize) -> usize {
    // (blueprint, time, resources, robots)
    type State<'blueprint> = (&'blueprint Blueprint, usize, [usize; 4], [usize; 4]);

    fn simulate(
        (blueprint, time, ores, robots): State,
        max_robots: [usize; 4],
        max_time: usize,
        max_geodes: &mut usize,
    ) {
        // Flag to check if we crafted something
        let mut crafted = false;
        for (i, recipe) in blueprint.recipies.iter().enumerate() {
            // Skip recipe, if we don't need new robots
            if robots[i] == max_robots[i] {
                continue;
            }

            // Skip recipe if we have some resource which we do not gather yet
            if recipe
                .iter()
                .zip(&robots)
                .any(|(&amount, &robots)| amount != 0 && robots == 0)
            {
                continue;
            }

            // We need to wait some time to gather enough ores
            let wait_time = recipe
                .iter()
                .enumerate()
                .filter(|&(_, &amount)| amount != 0)
                .map(|(i, &amount)| {
                    if amount <= ores[i] {
                        0
                    } else {
                        let needed_ore = amount - ores[i];
                        (needed_ore + robots[i] - 1) / robots[i]
                    }
                })
                .max()
                .unwrap();
            let new_time = time + wait_time + 1;
            if new_time >= max_time {
                continue;
            }

            let new_ores: [usize; 4] = ores
                .into_iter()
                .enumerate()
                .map(|(i, ore)| ore + robots[i] * (wait_time + 1) - recipe[i])
                .collect_vec()
                .try_into()
                .unwrap();
            let mut new_robots = robots;
            new_robots[i] += 1;

            // Predicting if we can gather more than max
            let remaining_time = max_time - new_time;
            let new_geodes = remaining_time * new_robots[3];
            let new_geodes_from_new_robots = (remaining_time * (remaining_time - 1)) / 2;
            if new_ores[3] + new_geodes + new_geodes_from_new_robots < *max_geodes {
                continue;
            }

            let new_state = (blueprint, new_time, new_ores, new_robots);
            crafted = true;
            simulate(new_state, max_robots, max_time, max_geodes);
        }

        // If we haven't crafted anything -> we can set new max geodes
        if !crafted {
            let geodes = ores[3] + robots[3] * (max_time - time);
            *max_geodes = geodes.max(*max_geodes);
        }
    }

    let initial_state = (blueprint, 0, [0; 4], [1, 0, 0, 0]);
    let mut max_robots: [usize; 4] = (0..4)
        .map(|i| {
            blueprint
                .recipies
                .iter()
                .map(|resources| resources[i])
                .max()
                .unwrap()
        })
        .collect_vec()
        .try_into()
        .unwrap();
    // Geodes max are always zero -> setting it to "infinity"
    max_robots[3] = std::usize::MAX;

    let mut max_geodes = 0;
    simulate(initial_state, max_robots, max_time, &mut max_geodes);

    max_geodes
}

fn main() {
    let input = get_input_text(DAY);
    let blueprints = input.lines().map(Blueprint::from).collect_vec();

    let solution1: Solution = blueprints
        .iter()
        .enumerate()
        .map(|(i, blueprint)| (i, simulate_blueprint(blueprint, 24)))
        .map(|(i, geodes)| geodes * (i + 1))
        .sum();

    let solution2: Solution = blueprints
        .iter()
        .take(3)
        .map(|blueprint| simulate_blueprint(blueprint, 32))
        .product();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
