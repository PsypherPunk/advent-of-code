use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Chemical {
    name: String,
    quantity: usize,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Reaction {
    inputs: Vec<Chemical>,
    product: Chemical,
}

impl Reaction {
    fn from_string(input: &str) -> Self {
        let input_output: Vec<&str> = input.split("=>").collect();
        let product = Chemical::from_input_string(input_output.last().unwrap());
        let inputs = input_output
            .first()
            .unwrap()
            .split(',')
            .map(|input| Chemical::from_input_string(input))
            .collect::<Vec<Chemical>>();
        Reaction { inputs, product }
    }
}

impl Chemical {
    fn from_input_string(input: &str) -> Self {
        let quantity_name = input.trim().split_ascii_whitespace().collect::<Vec<&str>>();

        Chemical {
            name: String::from(quantity_name.last().unwrap().trim()),
            quantity: quantity_name.first().unwrap().parse::<usize>().unwrap(),
        }
    }
}

fn get_fuel_per_trillion(reactions: &HashMap<String, Reaction>) -> usize {
    let mut current = 0;
    let mut increment = 1_000_000_000_000 / 100;
    loop {
        if get_fuel(reactions, current) > 1_000_000_000_000 {
            if increment == 1 {
                break;
            }
            current -= increment;
            increment /= 100;
        } else {
            current += increment;
        }
    }
    current - 1
}

fn get_fuel(reactions: &HashMap<String, Reaction>, fuel_count: usize) -> usize {
    let mut ore = 0;
    let mut spare = reactions
        .keys()
        .map(|k| (k.clone(), 0))
        .collect::<HashMap<String, usize>>();
    let mut need = vec![Chemical {
        name: String::from("FUEL"),
        quantity: fuel_count,
    }];
    while need.len() != 0 {
        let needed = need.pop().unwrap();
        if needed.name == String::from("ORE") {
            ore += needed.quantity;
        } else {
            let extra = *min(spare.get(&needed.name).unwrap_or(&0), &needed.quantity);
            let amount = needed.quantity - extra;
            spare.get_mut(&needed.name).map_or((), |e| *e -= extra);

            let reaction = reactions.get(&needed.name).unwrap();
            let ratio = (amount as f64 / reaction.product.quantity as f64).ceil();
            let materials = reaction.product.quantity as f64 * ratio;
            if materials > amount as f64 {
                spare
                    .get_mut(&needed.name)
                    .map_or((), |s| *s += (materials - amount as f64) as usize);
            }
            reaction.inputs.iter().for_each(|chemical| {
                need.push(Chemical {
                    name: chemical.name.clone(),
                    quantity: chemical.quantity * ratio as usize,
                })
            });
        }
    }

    ore + *spare.get(&"FUEL".to_owned()).unwrap_or(&0)
}

fn read_input() -> String {
    let filename = "input.txt";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        }
        Err(error) => {
            panic!("Error opening file {}: {}", filename, error);
        }
    }
}

fn get_reactions(input: &str) -> HashMap<String, Reaction> {
    let reactions = input
        .trim()
        .lines()
        .map(|line| {
            let reaction = Reaction::from_string(line);
            (reaction.product.name.clone(), reaction)
        })
        .collect::<Vec<(String, Reaction)>>();
    HashMap::from_iter(reactions)
}

fn main() {
    let input = read_input();
    let reactions = get_reactions(&input);
    let fuel = get_fuel(&reactions, 1);
    println!(
        "…what is the minimum amount of ORE required to produce exactly 1 FUEL? {}",
        fuel
    );

    let fuel_per_trillion = get_fuel_per_trillion(&reactions);
    println!(
        "…what is the maximum amount of FUEL you can produce? {}",
        fuel_per_trillion
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_31() {
        let input = String::from(
            r#"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"#,
        );
        let reactions = get_reactions(&input);
        let fuel = get_fuel(&reactions, 1);
        assert_eq!(fuel, 31);
    }

    #[test]
    fn test_165() {
        let input = String::from(
            r#"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"#,
        );
        let reactions = get_reactions(&input);
        let fuel = get_fuel(&reactions, 1);
        assert_eq!(fuel, 165);
    }

    #[test]
    fn test_13312() {
        let input = String::from(
            r#"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"#,
        );
        let reactions = get_reactions(&input);
        let fuel = get_fuel(&reactions, 1);
        assert_eq!(fuel, 13312);
    }

    #[test]
    fn test_180697() {
        let input = String::from(
            r#"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"#,
        );
        let reactions = get_reactions(&input);
        let fuel = get_fuel(&reactions, 1);
        assert_eq!(fuel, 180697);
    }

    #[test]
    fn test_2210736() {
        let input = String::from(
            r#"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"#,
        );
        let reactions = get_reactions(&input);
        let fuel = get_fuel(&reactions, 1);
        assert_eq!(fuel, 2210736);
    }
}
