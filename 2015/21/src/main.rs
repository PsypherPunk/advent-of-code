use std::cmp::max;
use std::fs;
use std::iter;

#[derive(Debug, PartialEq)]
struct Item {
    name: String,
    cost: i16,
    damage: i16,
    armour: i16,
}

#[derive(Clone, Debug)]
struct Attacker {
    hit_points: i16,
    damage: i16,
    armour: i16,
}

#[derive(Debug)]
struct Outcome {
    cost: i16,
    win: bool,
}

impl Attacker {
    fn from_string(input: &str) -> Self {
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
            armour: input[2],
        }
    }

    fn attack(&self, opponent: &Attacker) -> bool {
        let inflicted = max(1, self.damage - opponent.armour);
        let received = max(1, opponent.damage - self.armour);

        let mut self_hp = self.hit_points;
        let mut opponent_hp = opponent.hit_points;
        loop {
            opponent_hp -= inflicted;
            if opponent_hp <= 0 {
                break;
            }
            self_hp -= received;
            if self_hp <= 0 {
                break;
            }
        }
        self_hp > 0
    }
}

fn get_items(items: &str) -> Vec<Item> {
    items
        .trim()
        .lines()
        .skip(1)
        .map(|line| {
            let attributes = line
                .trim()
                .rsplitn(14, ' ')
                .filter(|c| !c.is_empty())
                .collect::<Vec<&str>>();
            Item {
                name: String::from(attributes[3].trim()),
                cost: attributes[2].parse().unwrap(),
                damage: attributes[1].parse().unwrap(),
                armour: attributes[0].parse().unwrap(),
            }
        })
        .collect()
}

fn get_weapons() -> Vec<Item> {
    let weapons = r#"Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0"#;

    get_items(weapons)
}

fn get_armour() -> Vec<Item> {
    let armour = r#"Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5"#;

    get_items(armour)
}

fn get_rings() -> Vec<Item> {
    let rings = r#"Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3"#;

    get_items(rings)
}

fn fight_with_items(items: &[&Item], boss: &Attacker) -> bool {
    let player = Attacker {
        hit_points: 100,
        damage: items.iter().map(|item| item.damage).sum(),
        armour: items.iter().map(|item| item.armour).sum(),
    };
    player.attack(boss)
}

fn get_outcomes(boss: &Attacker) -> Vec<Outcome> {
    let weapons = get_weapons();
    let armours = get_armour();
    let rings = get_rings();
    let none = Item {
        name: "".to_string(),
        cost: 0,
        damage: 0,
        armour: 0,
    };

    let mut outcomes = Vec::new();
    for ring_one in rings.iter().chain(iter::once(&none)) {
        for ring_two in rings.iter().chain(iter::once(&none)) {
            for armour in armours.iter().chain(iter::once(&none)) {
                for weapon in weapons.iter() {
                    let items = vec![ring_one, ring_two, armour, weapon];
                    outcomes.push(Outcome {
                        cost: items.iter().map(|item| item.cost).sum(),
                        win: fight_with_items(&items, &boss),
                    });
                }
            }
        }
    }

    outcomes
}

fn find_cheapest_win(boss: &Attacker) -> i16 {
    let outcomes = get_outcomes(boss);

    outcomes
        .iter()
        .filter(|outcome| outcome.win)
        .map(|outcome| outcome.cost)
        .min()
        .unwrap()
}

fn find_most_expensive_loss(boss: &Attacker) -> i16 {
    let outcomes = get_outcomes(boss);

    outcomes
        .iter()
        .filter(|outcome| !outcome.win)
        .map(|outcome| outcome.cost)
        .max()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let boss = Attacker::from_string(&input);

    println!(
        "What is the least amount of gold you can spend and still win the fight? {}",
        find_cheapest_win(&boss),
    );

    println!(
        "What is the most amount of gold you can spend and still lose the fight? {}",
        find_most_expensive_loss(&boss),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let weapons = get_weapons();
        let armour = get_armour();
        let rings = get_rings();

        let mut player = Attacker {
            hit_points: 8,
            damage: 5,
            armour: 5,
        };
        let mut boss = Attacker {
            hit_points: 12,
            damage: 7,
            armour: 2,
        };

        assert!(player.attack(&boss));
    }
}
