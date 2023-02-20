use std::collections::HashMap;

use aoc_2015::*;

const DAY: i32 = 22;
type Solution = usize;

const SPELLS: [Spell; 5] = [
    Spell::Missile,
    Spell::Drain,
    Spell::Shield,
    Spell::Poison,
    Spell::Recharge,
];

enum Spell {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> Solution {
        match self {
            Self::Missile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }
}

const PLAYER_HP: isize = 50;
const PLAYER_MANA: usize = 500;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Game {
    hard: bool,

    player_hp: isize,
    player_mana: usize,

    boss_hp: isize,
    boss_damage: isize,

    shield_time: isize,
    poison_time: isize,
    recharge_time: isize,
}

impl Game {
    fn new(input: &str, hard: bool) -> Self {
        let mut boss_stats = input
            .lines()
            .map(|line| line.split_once(": ").unwrap().1)
            .map(|stat| stat.parse().unwrap());

        Self {
            hard,

            player_hp: PLAYER_HP - hard as isize,
            player_mana: PLAYER_MANA,

            boss_hp: boss_stats.next().unwrap(),
            boss_damage: boss_stats.next().unwrap(),

            shield_time: 0,
            poison_time: 0,
            recharge_time: 0,
        }
    }

    fn is_game_over(&self) -> Option<bool> {
        if self.player_hp <= 0 {
            Some(false)
        } else if self.boss_hp <= 0 {
            Some(true)
        } else {
            None
        }
    }

    fn make_turn(&mut self, spell: &Spell) -> Option<bool> {
        // Player's turn
        self.cast_spell(spell);
        self.tick_effects();
        if let Some(winner) = self.is_game_over() {
            return Some(winner);
        }

        // Boss' turn
        let armor = if self.shield_time > 0 { 7 } else { 0 };
        let boss_attack = 1.max(self.boss_damage - armor);
        self.player_hp -= boss_attack;
        if let Some(winner) = self.is_game_over() {
            return Some(winner);
        }

        // If we are in hard mode, we need to damage player
        if self.hard {
            self.player_hp -= 1;
        }

        // After boss' turn we also have to tick effects
        self.tick_effects();
        self.is_game_over()
    }

    fn cast_spell(&mut self, spell: &Spell) {
        //// Some "incorrect" casts of spells shouldn't appear here, if spells() was used :wink:
        // Using player's mana
        self.player_mana -= spell.cost();
        // Doing spell effects
        match spell {
            Spell::Missile => self.boss_hp -= 4,
            Spell::Shield => self.shield_time = 6,
            Spell::Poison => self.poison_time = 6,
            Spell::Recharge => self.recharge_time = 5,
            Spell::Drain => {
                self.boss_hp -= 2;
                self.player_hp += 2;
            }
        }
    }

    fn tick_effects(&mut self) {
        if self.shield_time > 0 {
            self.shield_time -= 1;
        }

        if self.poison_time > 0 {
            self.poison_time -= 1;
            self.boss_hp -= 3;
        }

        if self.recharge_time > 0 {
            self.recharge_time -= 1;
            self.player_mana += 101;
        }
    }

    fn spells(&self) -> impl Iterator<Item = &Spell> {
        SPELLS
            .iter()
            .filter(|spell| spell.cost() <= self.player_mana)
            .filter(|spell| match spell {
                Spell::Shield => self.shield_time == 0,
                Spell::Poison => self.poison_time == 0,
                Spell::Recharge => self.recharge_time == 0,
                _ => true,
            })
    }
}

fn solve(games: &mut HashMap<Game, Solution>, game: Game, used_mana: Solution) -> Solution {
    // Optimization: If we had the same game state with less or equal mana -> lose
    if let Some(mana) = games.get(&game) {
        if *mana <= used_mana {
            return std::usize::MAX;
        }
    }

    // Adding this game state to hashmap
    games.insert(game.clone(), used_mana);

    // Player has run out of mana -> lose
    if game.spells().count() == 0 {
        return std::usize::MAX;
    }

    game.spells()
        .map(|spell| {
            let mut new_game = game.clone();
            let new_used_mana = used_mana + spell.cost();
            match new_game.make_turn(spell) {
                Some(victory) if victory => return new_used_mana,
                None => solve(games, new_game, new_used_mana),
                // We lost in the game
                _ => std::usize::MAX,
            }
        })
        .min()
        .unwrap()
}

fn main() {
    let input = get_input_text(DAY);
    let solution1: Solution = solve(&mut HashMap::new(), Game::new(input.trim(), false), 0);
    let solution2: Solution = solve(&mut HashMap::new(), Game::new(input.trim(), true), 0);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
