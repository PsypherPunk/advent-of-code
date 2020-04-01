use std::cmp::max;
use std::collections::VecDeque;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum SpellName {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Spell {
    name: SpellName,
    cost: i16,
    damage: i16,
    hit_points: i16,
    armour: i16,
    duration: i16,
    mana: i16,
}

#[derive(Clone, Copy, Debug)]
struct Attacker {
    hit_points: i16,
    damage: i16,
    armour: i16,
    mana: i16,
}

#[derive(Clone, Debug)]
struct Round {
    player: Attacker,
    boss: Attacker,
    mana_spent: i16,
    spell: Spell,
    effects: Vec<Spell>,
    cast: Vec<SpellName>,
}

fn get_spells() -> Vec<Spell> {
    vec![
        Spell {
            name: SpellName::MagicMissile,
            cost: 53,
            damage: 4,
            hit_points: 0,
            armour: 0,
            duration: 0,
            mana: 0,
        },
        Spell {
            name: SpellName::Drain,
            cost: 73,
            damage: 2,
            hit_points: 2,
            armour: 0,
            duration: 0,
            mana: 0,
        },
        Spell {
            name: SpellName::Shield,
            cost: 113,
            damage: 0,
            hit_points: 0,
            armour: 7,
            duration: 6,
            mana: 0,
        },
        Spell {
            name: SpellName::Poison,
            cost: 173,
            damage: 3,
            hit_points: 0,
            armour: 0,
            duration: 6,
            mana: 0,
        },
        Spell {
            name: SpellName::Recharge,
            cost: 229,
            damage: 0,
            hit_points: 0,
            armour: 0,
            duration: 5,
            mana: 101,
        },
    ]
}

impl Attacker {
    fn new() -> Self {
        Attacker {
            hit_points: 0,
            damage: 0,
            armour: 0,
            mana: 0,
        }
    }

    fn player() -> Self {
        Attacker {
            hit_points: 50,
            mana: 500,
            ..Attacker::new()
        }
    }

    fn boss(input: &str) -> Self {
        let input = input
            .trim()
            .lines()
            .map(|line| {
                let data = line.trim().split_whitespace().collect::<Vec<&str>>();
                data.last().unwrap().parse::<i16>().unwrap()
            })
            .collect::<Vec<i16>>();

        Attacker {
            hit_points: input[0],
            damage: input[1],
            ..Attacker::new()
        }
    }
}

impl Round {
    fn apply_effects(&mut self) {
        let mut armour = 0;
        for i in 0..self.effects.len() {
            self.effects[i].duration -= 1;
            self.boss.hit_points -= self.effects[i].damage;
            self.player.mana += self.effects[i].mana;
            armour = max(armour, self.effects[i].armour);
        }
        self.player.armour = armour;
        self.effects.retain(|effect| effect.duration > 0);
    }

    fn cast_spell(&mut self) {
        if self.spell.duration > 0 {
            self.effects.push(self.spell);
        } else {
            self.boss.hit_points -= self.spell.damage;
            self.player.hit_points += self.spell.hit_points;
            self.player.armour = max(self.player.armour, self.spell.armour);
        }
        self.player.mana -= self.spell.cost;
        self.mana_spent += self.spell.cost;
    }
}

fn bfs(input: &str, hard: bool) -> Option<i16> {
    let mut queue = VecDeque::new();

    get_spells().iter().for_each(|spell| {
        queue.push_back(Round {
            player: Attacker::player(),
            boss: Attacker::boss(input),
            mana_spent: 0,
            spell: *spell,
            effects: Vec::new(),
            cast: Vec::new(),
        })
    });

    let mut least_mana: Option<i16> = None;

    while !queue.is_empty() {
        let mut round = queue.pop_front().unwrap();

        // -- Player turn --
        // If this brings you to or below 0 hit points, you lose.
        if hard {
            round.player.hit_points -= 1;
            if round.player.hit_points <= 0 {
                continue;
            }
        }

        round.apply_effects();
        // If boss dead, win.
        if round.boss.hit_points <= 0 {
            if least_mana.is_none() || round.mana_spent < least_mana.unwrap() {
                least_mana = Some(round.mana_spent);
            }
            continue;
        }

        //  If you cannot afford to cast any spell, you lose.
        if round.player.mana < round.spell.cost {
            continue;
        }
        round.cast_spell();
        round.cast.push(round.spell.name);
        if least_mana.is_some() && round.mana_spent >= least_mana.unwrap() {
            continue;
        }

        // If boss dead, win.
        if round.boss.hit_points <= 0 {
            if least_mana.is_none() || round.mana_spent < least_mana.unwrap() {
                least_mana = Some(round.mana_spent);
            }
            continue;
        }

        // -- Boss turn --
        round.apply_effects();
        // If boss dead, win.
        if round.boss.hit_points <= 0 {
            if least_mana.is_none() || round.mana_spent < least_mana.unwrap() {
                least_mana = Some(round.mana_spent);
            }
            continue;
        }
        round.player.hit_points -= max(round.boss.damage - round.player.armour, 1);

        // If player dead, lose.
        if round.player.hit_points <= 0 {
            continue;
        }

        get_spells()
            .iter()
            .filter(|&spell| {
                !round
                    .effects
                    .iter()
                    .filter(|effect| effect.duration > 1)
                    .map(|effect| effect.name)
                    .any(|spell_name| spell_name == spell.name)
            })
            .for_each(|spell| {
                queue.push_back(Round {
                    player: round.player,
                    boss: round.boss,
                    mana_spent: round.mana_spent,
                    spell: *spell,
                    effects: round.effects.clone(),
                    cast: round.cast.clone(),
                })
            });
    }

    least_mana
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the least amount of mana you can spend and still win the fight? {}",
        bfs(&input, false).unwrap(),
    );

    println!(
        "…what is the least amount of mana you can spend and still win the fight? {}",
        bfs(&input, true).unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {}
}
