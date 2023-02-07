use std::collections::{HashMap, HashSet};

use aoc_2015::*;
use itertools::Itertools;

const DAY: i32 = 9;
type Solution = usize;
type City<'input> = &'input str;
type Route<'input> = (City<'input>, City<'input>);

fn main() {
    let input = get_input_text(DAY);
    let mut cities: HashSet<City> = HashSet::new();
    let mut distances: HashMap<Route, Solution> = HashMap::new();

    for route in input.lines() {
        let (city_pair, distance) = route.split_once(" = ").unwrap();
        let (from, to) = city_pair.split_once(" to ").unwrap();
        let distance = distance.parse().unwrap();

        cities.insert(from);
        cities.insert(to);
        distances.insert((from, to), distance);
        distances.insert((to, from), distance);
    }

    let results = cities.iter().permutations(cities.len()).map(|route| {
        route
            .windows(2)
            .map(|pair| distances.get(&(pair[0], pair[1])).unwrap())
            .sum()
    });

    let (solution1, solution2): (Solution, Solution) = results.minmax().into_option().unwrap();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
