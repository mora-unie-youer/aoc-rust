use std::cmp::Reverse;

use aoc_2018::*;

const DAY: i32 = 24;
type Solution = usize;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Team {
    Immune,
    Infect,
}

#[derive(Clone)]
struct Group<'input> {
    units: usize,

    health: usize,
    immunity: Vec<&'input str>,
    weakness: Vec<&'input str>,

    damage: usize,
    damage_type: &'input str,
    initiative: usize,
}

impl Group<'_> {
    fn damage_to(&self, other: &Group) -> usize {
        let power = self.power();

        if other.immunity.contains(&self.damage_type) {
            0
        } else if other.weakness.contains(&self.damage_type) {
            power * 2
        } else {
            power
        }
    }

    fn power(&self) -> usize {
        self.units * self.damage
    }
}

impl<'input> From<&'input str> for Group<'input> {
    fn from(value: &'input str) -> Self {
        let regex = regex!(
            r"(\d+) units each with (\d+) hit points(?: \((.*)\))? with an attack that does (\d+) (\w+) damage at initiative (\d+)"
        );

        let captures = regex.captures(value).unwrap();
        let units = captures.get(1).unwrap().as_str().parse().unwrap();
        let health = captures.get(2).unwrap().as_str().parse().unwrap();

        let (immunity, weakness) = if let Some(special) = captures.get(3) {
            let special = special.as_str();
            match special.split_once("; ") {
                Some((a, b)) => {
                    let (immunity, weakness) = if a.starts_with('i') { (a, b) } else { (b, a) };
                    (
                        immunity[10..].split(", ").collect(),
                        weakness[8..].split(", ").collect(),
                    )
                }
                None if special.starts_with('i') => (special[10..].split(", ").collect(), vec![]),
                None => (vec![], special[8..].split(", ").collect()),
            }
        } else {
            (vec![], vec![])
        };

        let damage = captures.get(4).unwrap().as_str().parse().unwrap();
        let damage_type = captures.get(5).unwrap().as_str();
        let initiative = captures.get(6).unwrap().as_str().parse().unwrap();

        Self {
            units,

            health,
            immunity,
            weakness,

            damage,
            damage_type,
            initiative,
        }
    }
}

type Unit<'input> = (Team, Group<'input>);
fn round(armies: &mut Vec<Unit>) -> bool {
    armies.sort_by_key(|(_, group)| Reverse((group.power(), group.initiative)));
    let mut targets = vec![None; armies.len()];

    for (i, (team, group)) in armies.iter().enumerate() {
        //// This code for some reason doesn't work
        //// (I guess this is due to implementation of `.max_by_key()`)
        // targets[i] = armies
        //     .iter()
        //     .enumerate()
        //     .filter(|&(j, unit)| &unit.0 != team && !targets.contains(&Some(j)))
        //     .max_by_key(|(_, unit)| group.damage_to(&unit.1))
        //     .map(|(j, _)| j);

        let mut largest = 0;
        for (j, target) in armies.iter().enumerate() {
            if team == &target.0 || targets.contains(&Some(j)) {
                continue;
            }

            let damage = group.damage_to(&target.1);
            if damage > largest {
                largest = damage;
                targets[i] = Some(j);
            }
        }
    }

    let mut did_damage = false;
    let mut attackers: Vec<_> = (0..armies.len()).collect();
    attackers.sort_by_key(|&i| Reverse(armies[i].1.initiative));
    for i in attackers {
        let attacker = &armies[i].1;
        if attacker.units == 0 {
            continue;
        }

        if let Some(j) = targets[i] {
            let defender = &armies[j].1;
            let damage = attacker.damage_to(defender);

            let defender = &mut armies[j].1;
            defender.units = defender.units.saturating_sub(damage / defender.health);
            did_damage |= damage > defender.health;
        }
    }

    armies.retain(|(_, group)| group.units > 0);
    did_damage
}

fn battle(mut armies: Vec<Unit>) -> (Option<Team>, usize) {
    loop {
        if !round(&mut armies) {
            // Did no damage -> Draw
            break (None, 0);
        }

        let immune = armies.iter().find(|(team, _)| team == &Team::Immune);
        let infect = armies.iter().find(|(team, _)| team == &Team::Infect);

        if immune.is_none() {
            let units = armies.iter().map(|(_, group)| group.units).sum();
            break (Some(Team::Infect), units);
        } else if infect.is_none() {
            let units = armies.iter().map(|(_, group)| group.units).sum();
            break (Some(Team::Immune), units);
        }
    }
}

fn main() {
    let input = get_input_text(DAY);
    let (immune, infect) = input.split_once("\n\n").unwrap();

    let mut armies: Vec<Unit> = vec![];
    armies.extend(immune.lines().skip(1).map(|v| (Team::Immune, v.into())));
    let immunes = armies.len();
    armies.extend(infect.lines().skip(1).map(|v| (Team::Infect, v.into())));

    let solution1: Solution = battle(armies.clone()).1;

    let solution2: Solution = (1..)
        .find_map(|boost| {
            let mut armies = armies.clone();
            armies[..immunes]
                .iter_mut()
                .for_each(|(_, group)| group.damage += boost);
            match battle(armies) {
                (Some(Team::Immune), units) => Some(units),
                _ => None,
            }
        })
        .unwrap();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::{battle, Team, Unit};

    const INPUT: &str = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    #[test]
    fn test_part1() {
        let (immune, infect) = INPUT.split_once("\n\n").unwrap();

        let mut armies: Vec<Unit> = vec![];
        armies.extend(immune.lines().skip(1).map(|v| (Team::Immune, v.into())));
        armies.extend(infect.lines().skip(1).map(|v| (Team::Infect, v.into())));
        assert_eq!(battle(armies), (Some(Team::Infect), 5216));
    }

    #[test]
    fn test_part2() {
        let (immune, infect) = INPUT.split_once("\n\n").unwrap();

        let mut armies: Vec<Unit> = vec![];
        armies.extend(immune.lines().skip(1).map(|v| (Team::Immune, v.into())));
        let immunes = armies.len();
        armies.extend(infect.lines().skip(1).map(|v| (Team::Infect, v.into())));

        armies[..immunes]
            .iter_mut()
            .for_each(|(_, group)| group.damage += 1570);
        assert_eq!(battle(armies), (Some(Team::Immune), 51));
    }
}
