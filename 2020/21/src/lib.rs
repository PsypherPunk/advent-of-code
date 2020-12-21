use std::collections::{HashMap, HashSet};
use std::str::FromStr;

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
            let (ingredients, allergens) = match line.split('(').collect::<Vec<_>>()[..] {
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
                    let allergen_ingredients = all_allergens
                        .entry(allergen.trim().to_string())
                        .or_insert(ingredients.clone());
                    *allergen_ingredients = ingredients
                        .intersection(allergen_ingredients)
                        .cloned()
                        .collect();
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
        let allergen_ingredients = self.allergens.values().flatten().collect::<HashSet<_>>();

        self.ingredients
            .iter()
            .filter(|ingredient| !allergen_ingredients.contains(*ingredient))
            .count()
    }

    pub fn get_canonical_dangerous_ingredients(&self) -> String {
        let mut to_check = self.allergens.clone();
        let mut canonical_dangerous_ingredients = Vec::new();

        while let Some((allergen, ingredient)) = to_check
            .iter()
            .filter_map(|(allergen, ingredients)| match ingredients.len() {
                1 => Some((allergen.clone(), ingredients.iter().next().unwrap().clone())),
                _ => None,
            })
            .next()
        {
            to_check.remove(&allergen).unwrap();
            to_check.values_mut().for_each(|ingredients| {
                ingredients.remove(&ingredient);
            });
            canonical_dangerous_ingredients.push((allergen, ingredient));
        }

        canonical_dangerous_ingredients.sort_unstable_by(|(a, _), (b, _)| a.cmp(&b));
        canonical_dangerous_ingredients
            .iter()
            .map(|(_, ingredient)| ingredient.as_str())
            .collect::<Vec<_>>()
            .join(",")
    }
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

        assert_eq!(5, food.get_safe_count());
    }

    #[test]
    fn test_part_two() {
        let food = Food::from_str(&INPUT).unwrap();

        assert_eq!(
            "mxmxvkd,sqjhc,fvjkl",
            food.get_canonical_dangerous_ingredients()
        );
    }
}
