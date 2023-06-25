use std::collections::{HashMap, HashSet};

use aoc_2020::*;

const DAY: i32 = 21;
type Solution = String;

struct Food<'input> {
    ingridients: HashSet<&'input str>,
    allergens: HashSet<&'input str>,
}

impl<'input> From<&'input str> for Food<'input> {
    fn from(value: &'input str) -> Self {
        let value = value.trim_end_matches(')');
        let (ingridients, allergens) = value.split_once(" (contains ").unwrap();

        let ingridients = ingridients.split_ascii_whitespace().collect();
        let allergens = allergens.split(", ").collect();
        Self {
            ingridients,
            allergens,
        }
    }
}

fn main() {
    let input = get_input_text(DAY);
    let foods: Vec<_> = input.lines().map(Food::from).collect();

    let allergic_ingridients = {
        let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();

        for food in &foods {
            for allergen in &food.allergens {
                let ingridients = map.entry(allergen).or_insert(food.ingridients.clone());
                *ingridients = ingridients
                    .intersection(&food.ingridients)
                    .cloned()
                    .collect();
            }
        }

        let mut allergic_ingridients: HashMap<&str, &str> = HashMap::new();
        while let Some((allergen, ingridients)) = map.iter().find(|(_, i)| i.len() == 1) {
            let ingridient = ingridients.iter().next().cloned().unwrap();
            allergic_ingridients.insert(ingridient, allergen);
            map.iter_mut().for_each(|(_, ingridients)| {
                ingridients.retain(|&ing| ing != ingridient);
            });
        }

        allergic_ingridients
    };

    let solution1: Solution = foods
        .iter()
        .flat_map(|food| &food.ingridients)
        .filter(|&ingridient| !allergic_ingridients.contains_key(ingridient))
        .count()
        .to_string();

    let solution2: Solution = {
        let mut dangerous_list: Vec<_> = allergic_ingridients.into_iter().collect();
        dangerous_list.sort_by_key(|&(_, allergen)| allergen);

        dangerous_list
            .into_iter()
            .map(|(ingridient, _)| ingridient)
            .collect::<Vec<_>>()
            .join(",")
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
