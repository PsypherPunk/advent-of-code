use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttackType {
    Radiation,
    Bludgeoning,
    Cold,
    Slashing,
    Fire,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum GroupType {
    ImmuneSystem,
    Infection,
}

#[derive(Clone, Debug)]
pub struct Group {
    type_: GroupType,
    units: usize,
    hit_points: usize,
    weaknesses: Vec<AttackType>,
    immunities: Vec<AttackType>,
    attack_damage: usize,
    attack_type: AttackType,
    initiative: usize,
}

pub struct Reindeer {
    groups: Vec<Group>,
}

impl Group {
    fn get_effective_power(&self) -> usize {
        self.units * self.attack_damage
    }

    fn set_units(&mut self, units: usize) {
        self.units = units
    }

    fn get_damage_to(&self, other: &Group) -> usize {
        let damage = self.get_effective_power();

        if other.weaknesses.contains(&self.attack_type) {
            damage * 2
        } else if other.immunities.contains(&self.attack_type) {
            0
        } else {
            damage
        }
    }
}

impl Reindeer {
    fn has_winner(&self) -> bool {
        let types_still_alive = self
            .groups
            .iter()
            .filter(|group| group.units > 0)
            .map(|group| group.type_)
            .collect::<BTreeSet<_>>();

        types_still_alive.len() == 1
    }

    fn order_target_selection_phase(&mut self) {
        self.groups.sort_by(|a, b| {
            b.get_effective_power()
                .cmp(&a.get_effective_power())
                .then(b.initiative.cmp(&a.initiative))
        });
    }

    fn get_state(&self) -> Vec<usize> {
        self.groups
            .iter()
            .map(|group| group.units)
            .collect::<Vec<_>>()
    }

    pub fn fight(&mut self) -> Result<usize, String> {
        while !self.has_winner() {
            let mut attacks = Vec::new();
            let mut targeted = BTreeSet::new();

            self.order_target_selection_phase();

            let initial_state = self.get_state();

            let living_groups = self
                .groups
                .iter()
                .enumerate()
                .filter(|(_, group)| group.units > 0);

            for (group_index, group) in living_groups {
                let mut possible_targets = self
                    .groups
                    .iter()
                    .enumerate()
                    .filter(|(target_index, target)| {
                        target.type_ != group.type_
                            && target.units > 0
                            && !targeted.contains(target_index)
                            && group.get_damage_to(target) > 0
                    })
                    .collect::<Vec<_>>();

                if !possible_targets.is_empty() {
                    possible_targets.sort_by(|(_, a), (_, b)| {
                        group
                            .get_damage_to(b)
                            .cmp(&group.get_damage_to(a))
                            .then(b.get_effective_power().cmp(&a.get_effective_power()))
                            .then(b.initiative.cmp(&a.initiative))
                    });

                    if let Some((target_index, _)) = possible_targets.get(0) {
                        attacks.push((group_index, *target_index));
                        targeted.insert(*target_index);
                    }
                }
            }

            attacks.sort_by(|(a, _), (b, _)| {
                self.groups[*b].initiative.cmp(&self.groups[*a].initiative)
            });

            for (attacker, defender) in attacks.iter() {
                let units = self.groups[*defender].units.saturating_sub(
                    self.groups[*attacker].get_damage_to(&self.groups[*defender])
                        / self.groups[*defender].hit_points,
                );
                self.groups[*defender].set_units(units);
            }
            if self.get_state() == initial_state {
                return Err("Attack round had no effect; stalemate.".to_owned());
            }
        }
        Ok(self.groups.iter().map(|group| group.units).sum())
    }

    fn apply_boost(&mut self, boost: usize) {
        self.groups
            .iter_mut()
            .filter(|group| matches!(group.type_, GroupType::ImmuneSystem))
            .for_each(|group| {
                group.attack_damage += boost;
            });
    }
}

pub fn get_immune_system_boost(input: &str) -> usize {
    let mut boosted_result = 0;

    for boost in 1.. {
        let mut reindeer: Reindeer = reindeer::reindeer(&input).unwrap();
        reindeer.apply_boost(boost);

        if let Ok(result) = reindeer.fight() {
            let types_still_alive = reindeer
                .groups
                .iter()
                .filter(|group| group.units > 0)
                .map(|group| group.type_)
                .collect::<BTreeSet<_>>();

            if matches!(
                types_still_alive.iter().next().unwrap(),
                GroupType::ImmuneSystem,
            ) {
                boosted_result = result;
                break;
            }
        }
    }

    boosted_result
}

peg::parser! {
    pub grammar reindeer() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("Invalid integer.")) }

        rule attack_type() -> AttackType
            = at:$(['a'..='z']+)
                {
                    match at {
                        "radiation" => AttackType::Radiation,
                        "bludgeoning" => AttackType::Bludgeoning,
                        "cold" => AttackType::Cold,
                        "slashing" => AttackType::Slashing,
                        "fire" => AttackType::Fire,
                        _ => unreachable!()
                    }
                }

        pub rule weak_immune() -> (&'input str, Vec<AttackType>)
            = a:$(['a'..='z']+) " to " attack_types:attack_type() ++ ", "
                { (a, attack_types) }

        pub rule weaknesses_immunities() -> (Vec<AttackType>, Vec<AttackType>)
            = "(" weak_immune:weak_immune() ++ "; " ")"
                {
                    match &weak_immune[..] {
                        [("immune", immunities), ("weak", weaknesses)] => (immunities.to_owned(), weaknesses.to_owned()),
                        [("weak", weaknesses), ("immune", immunities)] => (immunities.to_owned(), weaknesses.to_owned()),
                        [("immune", immunities)] => (immunities.to_owned(), Vec::new()),
                        [("weak", weaknesses)] => (Vec::new(), weaknesses.to_owned()),
                        _ => unreachable!(),
                    }
                }

        pub rule group() -> Group
            = units:integer() _ "units each with" _ hit_points:integer() _ "hit points" _
              weak_immune:weaknesses_immunities()? _ "with an attack that does" _ attack_damage:integer() _
              attack_type:attack_type() " damage at initiative " initiative:integer()
                {
                    let (immunities, weaknesses) = weak_immune.unwrap_or((Vec::new(), Vec::new()));
                    Group {
                        type_: GroupType::ImmuneSystem,
                        units,
                        hit_points,
                        weaknesses,
                        immunities,
                        attack_damage,
                        attack_type,
                        initiative,
                    }
                }

        pub rule reindeer() -> Reindeer
            = "Immune System:"
              _
              immune_system:group() ++ _
              _
              "Infection:"
              _
              infection:group() ++ _
              _
                {
                    let mut infection = infection
                        .into_iter()
                        .map(|infection| Group {
                            type_: GroupType::Infection,
                            ..infection
                        })
                        .collect::<Vec<_>>();

                    let mut groups = immune_system.clone();
                    groups.append(&mut infection);

                    Reindeer {
                        groups,
                    }
                }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"#;

    #[test]
    fn test_units_parsing() {
        let group: Group = reindeer::group(
            "18 units each with 729 hit points (weak to fire; immune to cold, slashing) with an attack that does 8 radiation damage at initiative 10",
        ).unwrap();

        assert_eq!(18, group.units);
        assert_eq!(729, group.hit_points);
        assert_eq!(vec![AttackType::Fire], group.weaknesses);
        assert_eq!(
            vec![AttackType::Cold, AttackType::Slashing],
            group.immunities,
        );
        assert_eq!(8, group.attack_damage);
        assert_eq!(AttackType::Radiation, group.attack_type);
        assert_eq!(10, group.initiative);
    }

    #[test]
    fn test_immune_system_parsing() {
        let reindeer: Reindeer = reindeer::reindeer(&INPUT).unwrap();

        assert_eq!(4, reindeer.groups.len());
        assert_eq!(
            2,
            reindeer
                .groups
                .iter()
                .filter(|g| matches!(g.type_, GroupType::ImmuneSystem))
                .collect::<Vec<_>>()
                .len()
        );
        assert_eq!(
            2,
            reindeer
                .groups
                .iter()
                .filter(|g| matches!(g.type_, GroupType::Infection))
                .collect::<Vec<_>>()
                .len()
        );
    }

    #[test]
    fn test_part_one() {
        let mut reindeer: Reindeer = reindeer::reindeer(&INPUT).unwrap();

        let result = reindeer.fight().unwrap();

        assert_eq!(5_216, result);
    }

    #[test]
    fn test_part_two() {
        let result = get_immune_system_boost(&INPUT);

        assert_eq!(51, result);
    }
}
