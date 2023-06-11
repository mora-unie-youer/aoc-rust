use std::collections::HashMap;

use aoc_2020::*;

const DAY: i32 = 7;
type Solution = usize;
type Bags<'input> = HashMap<&'input str, Vec<(usize, &'input str)>>;

fn parse_input(input: &str) -> Bags {
    input
        .lines()
        .map(|line| {
            let (bag, subbags) = line.split_once(" contain ").unwrap();

            let bag = bag.rsplit_once(' ').unwrap().0;
            let subbags = subbags
                .split(", ")
                .filter(|bag| !bag.starts_with("no"))
                .map(|bag| bag.split_once(' ').unwrap())
                .map(|(amount, bag)| (amount, bag.rsplit_once(' ').unwrap().0))
                .map(|(amount, bag)| (amount.parse().unwrap(), bag))
                .collect();

            (bag, subbags)
        })
        .collect()
}

fn contains_shiny_gold<'input>(
    bag: &'input str,
    bags: &Bags<'input>,
    cache: &mut HashMap<&'input str, bool>,
) -> bool {
    if !cache.contains_key(bag) {
        let contains = bags[bag]
            .iter()
            .any(|(_, bag)| contains_shiny_gold(bag, bags, cache));
        cache.insert(bag, contains);
    }

    cache[bag]
}

fn total_bags<'input>(bag: &'input str, bags: &Bags<'input>) -> usize {
    1 + bags[bag]
        .iter()
        .map(|(amount, bag)| amount * total_bags(bag, bags))
        .sum::<usize>()
}

fn main() {
    let input = get_input_text(DAY);
    let bags = parse_input(&input);

    let solution1: Solution = {
        let mut cache = HashMap::new();
        cache.insert("shiny gold", true);

        bags.keys()
            .filter(|bag| contains_shiny_gold(bag, &bags, &mut cache))
            .count()
            - 1
    };

    let solution2: Solution = total_bags("shiny gold", &bags) - 1;

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
