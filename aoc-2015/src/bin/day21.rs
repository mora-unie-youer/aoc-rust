#![feature(int_roundings)]
#![feature(once_cell)]

use std::{borrow::Borrow, ops::Add, sync::LazyLock};

use aoc_2015::*;
use itertools::Itertools;

const DAY: i32 = 21;
type Solution = isize;

#[derive(Clone, Copy, Default)]
struct Item {
    price: isize,
    damage: isize,
    armor: isize,
}

impl Item {
    fn new(price: isize, damage: isize, armor: isize) -> Self {
        Self {
            price,
            damage,
            armor,
        }
    }
}

impl Add for Item {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            price: self.price + rhs.price,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
        }
    }
}

struct Entity {
    hp: isize,
    damage: isize,
    armor: isize,
}

impl Entity {
    fn can_win(&self, enemy: &Entity, equipment: &Item) -> bool {
        let self_attack = 1.max(self.damage + equipment.damage - enemy.armor);
        let enemy_attack = 1.max(enemy.damage - (self.armor + equipment.armor));

        let self_turns = enemy.hp.div_ceil(self_attack);
        let enemy_turns = self.hp.div_ceil(enemy_attack);

        self_turns <= enemy_turns
    }
}

impl From<&str> for Entity {
    fn from(input: &str) -> Self {
        let mut stats = input
            .lines()
            .map(|line| line.split_once(": ").unwrap().1)
            .map(|stat| stat.parse().unwrap());

        Self {
            hp: stats.next().unwrap(),
            damage: stats.next().unwrap(),
            armor: stats.next().unwrap(),
        }
    }
}

static WEAPONS: LazyLock<Vec<Item>> = LazyLock::new(|| {
    vec![
        Item::new(8, 4, 0),
        Item::new(10, 5, 0),
        Item::new(25, 6, 0),
        Item::new(40, 7, 0),
        Item::new(74, 8, 0),
    ]
});

static ARMOR: LazyLock<Vec<Item>> = LazyLock::new(|| {
    vec![
        Item::default(), // No armor
        Item::new(13, 0, 1),
        Item::new(31, 0, 2),
        Item::new(53, 0, 3),
        Item::new(75, 0, 4),
        Item::new(102, 0, 5),
    ]
});

static RINGS: LazyLock<Vec<Item>> = LazyLock::new(|| {
    vec![
        Item::default(), // No ring
        Item::default(), // No ring
        Item::new(25, 1, 0),
        Item::new(50, 2, 0),
        Item::new(100, 3, 0),
        Item::new(20, 0, 1),
        Item::new(40, 0, 2),
        Item::new(80, 0, 3),
    ]
});

fn main() {
    let input = get_input_text(DAY);
    let boss: Entity = Entity::from(input.borrow());
    let player = Entity {
        hp: 100,
        armor: 0,
        damage: 0,
    };

    let (mut min_win, mut max_lose) = (std::isize::MAX, std::isize::MIN);
    for weapon in WEAPONS.iter() {
        for armor in ARMOR.iter() {
            for (ring1, ring2) in RINGS.iter().tuple_combinations() {
                let equipment = [weapon, armor, ring1, ring2]
                    .iter()
                    .fold(Item::default(), |acc, &&item| acc + item);
                if player.can_win(&boss, &equipment) {
                    min_win = min_win.min(equipment.price);
                } else {
                    max_lose = max_lose.max(equipment.price);
                }
            }
        }
    }

    let solution1: Solution = min_win;
    let solution2: Solution = max_lose;

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
