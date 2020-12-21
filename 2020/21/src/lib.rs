use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
pub struct Food {
    ingredients: Vec<String>,
    allergens: HashMap<String, HashSet<String>>,
}

impl FromStr for Food {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut all_ingredients = Vec::new();
        let mut all_allergens = HashMap::new();

        s.trim().lines().for_each(|line| {
            let (ingredients, allergens) = match line.splitn(2, '(').collect::<Vec<_>>()[..] {
                [a, b] => (a, b),
                _ => panic!("Invalid line: {}", line),
            };

            let ingredients = ingredients
                .trim()
                .split_whitespace()
                .map(|ingredient| ingredient.to_string())
                .collect::<HashSet<_>>();

            all_ingredients.extend(ingredients.clone());

            allergens[8..(allergens.len() - 1)]
                .split(',')
                .for_each(|allergen| {
                    all_allergens
                        .entry(allergen.to_string())
                        .and_modify(|allergen_ingredients: &mut HashSet<String>| {
                            allergen_ingredients
                                .retain(|ingredient| ingredients.contains(ingredient));
                        })
                        .or_insert_with(|| ingredients.clone());
                });
        });

        Ok(Food {
            ingredients: all_ingredients,
            allergens: all_allergens,
        })
    }
}

impl Food {
    pub fn get_safe_count(&self) -> usize {
        let allergens = self.allergens.values().flatten().collect::<HashSet<_>>();

        self.ingredients
            .iter()
            .filter(|ingredient| !allergens.contains(*ingredient))
            .count()
    }

    pub fn get_canonical_dangerous_ingredients() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;

    #[test]
    fn test_part_one() {
        let food = Food::from_str(&INPUT).unwrap();
        dbg!(&food);

        assert_eq!(5, food.get_safe_count());
    }
}
