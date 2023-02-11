use std::collections::{HashMap, HashSet};

use aoc_2015::*;
use itertools::Itertools;

const DAY: i32 = 13;
type Solution = isize;
type Person<'input> = &'input str;
type Pair<'input> = (Person<'input>, Person<'input>);

fn solve(people: &HashSet<Person>, relations: &HashMap<Pair, Solution>) -> Solution {
    people
        .iter()
        .permutations(people.len())
        // Optimization: Take only one "rotation" of table - first (n-1)! elements
        .take((1..people.len()).product())
        .map(|table| {
            table
                .iter()
                .zip(table.iter().cycle().skip(1))
                .map(|(p1, p2)| {
                    relations.get(&(p1, p2)).unwrap() + relations.get(&(p2, p1)).unwrap()
                })
                .sum()
        })
        .max()
        .unwrap()
}

fn main() {
    let input = get_input_text(DAY);

    let mut people: HashSet<Person> = HashSet::new();
    let mut relations: HashMap<Pair, Solution> = HashMap::new();
    for relation in input.lines() {
        let parts: Vec<_> = relation.split_ascii_whitespace().collect();
        let (person1, person2) = (parts[0], parts[10]);
        let person2 = &person2[..person2.len() - 1];
        let value: Solution = parts[3].parse().unwrap();
        let signum = match parts[2] {
            "gain" => 1,
            "lose" => -1,
            _ => unreachable!(),
        };

        people.insert(person1);
        relations.insert((person1, person2), value * signum);
    }

    let solution1: Solution = solve(&people, &relations);

    for person in people.iter() {
        relations.insert((person, "Me"), 0);
        relations.insert(("Me", person), 0);
    }
    people.insert("Me");
    let solution2: Solution = solve(&people, &relations);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
